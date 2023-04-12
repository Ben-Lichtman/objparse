pub type Result<T> = core::result::Result<T, Error>;

#[repr(u16)]
pub enum Error {
	PeHeaders,
	ExportTable,
	ImportTable,
}
