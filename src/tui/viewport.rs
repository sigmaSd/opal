use {
    super::{
        Buffer,
        Cell,
        Region
    },
    std::io::Write,
    crossterm::{
        terminal::{
            enable_raw_mode, disable_raw_mode,
            EnterAlternateScreen, LeaveAlternateScreen,
            Clear, ClearType
        },
        cursor::{
            self,
            MoveTo,
            MoveToNextLine
        },
        style::Print,
        QueueableCommand,
        Result,
    }
};

pub struct Viewport<W>
where W: std::io::Write
 {
    buffer: Buffer,
    pub writer: W,
}

impl <W: std::io::Write>Viewport<W> {
    pub fn new(region: Region, writer: W) -> Self {
        Self {
            buffer: Buffer::filled(region, Cell::new('x')),
            writer
        }
    }

    pub fn draw(&mut self) {
    	let mut w = &mut self.writer;
        for (pos, cell) in self.buffer.iter() {
            crossterm::queue!(w, MoveTo::new(pos.col, pos.row)).unwrap();
            w.queue(Print(cell)).unwrap();
        }

        self.writer.flush().unwrap();
    }

    pub fn init(&mut self) -> Result<()> {
        enable_raw_mode()?;
        self.writer.queue(EnterAlternateScreen)?;
        self.writer.queue(cursor::Hide)?;
        self.writer.queue(cursor::DisableBlinking)?;
        crossterm::queue!(self.writer, MoveTo::new(0, 0)).unwrap();

//        self.writer.queue(MoveTo::new(0, 0))?;

        self.writer.flush()?;

        Ok(())
    }

    pub fn release(&mut self) -> Result<()> {
        self.writer.queue(cursor::EnableBlinking)?;
        self.writer.queue(cursor::Show)?;
        self.writer.queue(LeaveAlternateScreen)?;

        self.writer.flush()?;

        disable_raw_mode()
    }
}
