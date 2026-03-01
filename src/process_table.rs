use std::cmp::Ordering;

use iced::widget::operation::scroll_to;
use iced::widget::scrollable::AbsoluteOffset;
use iced::widget::{container, responsive, text};
use iced::{Element, Length, Renderer, Task};
use iced_table::table;
use sysinfo::System;

use crate::theme::{TableStyle, Theme};

pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
    pub disk_read_bytes: u64,
    pub disk_written_bytes: u64,
}

pub enum ProcessColumnKind {
    Pid,
    Name,
    CpuUsage,
    MemoryUsage,
    DiskUsage,
}

pub struct ProcessColumn {
    kind: ProcessColumnKind,
    width: f32,
    resize_offset: Option<f32>,
}

impl ProcessColumn {
    pub fn new(kind: ProcessColumnKind) -> Self {
        let width = match kind {
            ProcessColumnKind::Pid => 80.0,
            ProcessColumnKind::Name => 250.0,
            ProcessColumnKind::CpuUsage => 100.0,
            ProcessColumnKind::MemoryUsage => 120.0,
            ProcessColumnKind::DiskUsage => 150.0,
        };
        Self {
            kind,
            width,
            resize_offset: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProcessTableMessage {
    SyncHeader(AbsoluteOffset),
    Resizing(usize, f32),
    Resized,
    RowSelected(u32),
}

pub struct ProcessTableState {
    pub columns: Vec<ProcessColumn>,
    pub rows: Vec<ProcessInfo>,
    pub selected_pid: Option<u32>,
    pub header: iced::widget::Id,
    pub body: iced::widget::Id,
    pub footer: iced::widget::Id,
}

impl Default for ProcessTableState {
    fn default() -> Self {
        Self {
            columns: vec![
                ProcessColumn::new(ProcessColumnKind::Pid),
                ProcessColumn::new(ProcessColumnKind::Name),
                ProcessColumn::new(ProcessColumnKind::CpuUsage),
                ProcessColumn::new(ProcessColumnKind::MemoryUsage),
                ProcessColumn::new(ProcessColumnKind::DiskUsage),
            ],
            rows: Vec::new(),
            selected_pid: None,
            header: iced::widget::Id::unique(),
            body: iced::widget::Id::unique(),
            footer: iced::widget::Id::unique(),
        }
    }
}

pub fn update(
    state: &mut ProcessTableState,
    message: ProcessTableMessage,
) -> Task<ProcessTableMessage> {
    match message {
        ProcessTableMessage::SyncHeader(offset) => {
            return Task::batch(vec![
                scroll_to(state.header.clone(), offset),
                scroll_to(state.footer.clone(), offset),
            ]);
        }
        ProcessTableMessage::Resizing(index, offset) => {
            if let Some(col) = state.columns.get_mut(index) {
                col.resize_offset = Some(offset);
            }
        }
        ProcessTableMessage::Resized => {
            for col in &mut state.columns {
                if let Some(offset) = col.resize_offset.take() {
                    col.width += offset;
                }
            }
        }
        ProcessTableMessage::RowSelected(pid) => {
            state.selected_pid = Some(pid);
        }
    }
    Task::none()
}

pub fn view(state: &ProcessTableState) -> Element<'_, ProcessTableMessage, Theme> {
    let selected_row = state.selected_pid.and_then(|pid| {
        state.rows.iter().position(|r| r.pid == pid)
    });

    responsive(move |size| {
        let table = table(
            state.header.clone(),
            state.body.clone(),
            &state.columns,
            &state.rows,
            ProcessTableMessage::SyncHeader,
        )
        .on_column_resize(ProcessTableMessage::Resizing, ProcessTableMessage::Resized)
        .min_width(size.width)
        .style(TableStyle { selected_row });

        table.into()
    })
    .into()
}

impl<'a> table::Column<'a, ProcessTableMessage, Theme, Renderer> for ProcessColumn {
    type Row = ProcessInfo;

    fn header(&'a self, _col_index: usize) -> Element<'a, ProcessTableMessage, Theme> {
        let label = match self.kind {
            ProcessColumnKind::Pid => "PID",
            ProcessColumnKind::Name => "Name",
            ProcessColumnKind::CpuUsage => "CPU %",
            ProcessColumnKind::MemoryUsage => "Memory",
            ProcessColumnKind::DiskUsage => "Disk (R+W)",
        };

        container(text(label).size(14)).height(24).into()
    }

    fn cell(
        &'a self,
        _col_index: usize,
        _row_index: usize,
        row: &'a ProcessInfo,
    ) -> Element<'a, ProcessTableMessage, Theme> {
        let content: Element<'_, ProcessTableMessage, Theme> = match self.kind {
            ProcessColumnKind::Pid => text(row.pid).size(13).into(),

            ProcessColumnKind::Name => text(&row.name)
                .size(13)
                .wrapping(text::Wrapping::None)
                .into(),

            ProcessColumnKind::CpuUsage => text(format!("{:.1}%", row.cpu_usage)).size(13).into(),

            ProcessColumnKind::MemoryUsage => {
                let mb = row.memory_bytes as f64 / 1_048_576.0;
                let label = if mb >= 1024.0 {
                    format!("{:.1} GB", mb / 1024.0)
                } else {
                    format!("{:.1} MB", mb)
                };
                text(label).size(13).into()
            }

            ProcessColumnKind::DiskUsage => {
                let total = row.disk_read_bytes + row.disk_written_bytes;
                let mb = total as f64 / 1_048_576.0;
                let label = if mb >= 1024.0 {
                    format!("{:.1} GB", mb / 1024.0)
                } else {
                    format!("{:.1} MB", mb)
                };
                text(label).size(13).into()
            }
        };

        let pid = row.pid;
        iced::widget::mouse_area(container(content).width(Length::Fill))
            .on_press(ProcessTableMessage::RowSelected(pid))
            .into()
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}

pub fn collect_processes(sys: &System) -> Vec<ProcessInfo> {
    let mut procs: Vec<ProcessInfo> = sys
        .processes()
        .values()
        .map(|p| ProcessInfo {
            pid: p.pid().as_u32(),
            name: p.name().to_string_lossy().to_string(),
            cpu_usage: p.cpu_usage(),
            memory_bytes: p.memory(),
            disk_read_bytes: p.disk_usage().read_bytes,
            disk_written_bytes: p.disk_usage().written_bytes,
        })
        .collect();

    procs.sort_by(|a, b| {
        b.cpu_usage
            .partial_cmp(&a.cpu_usage)
            .unwrap_or(Ordering::Equal)
    });

    procs
}
