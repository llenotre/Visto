//! This module implements Graphics Contexts (GC).

/// TODO doc
#[derive(Clone)]
pub enum Function {
	/// TODO doc
	Clear,
	/// TODO doc
	And,
	/// TODO doc
	AndReverse,
	/// TODO doc
	Copy,
	/// TODO doc
	AndInverted,
	/// TODO doc
	NoOp,
	/// TODO doc
	Xor,
	/// TODO doc
	Or,
	/// TODO doc
	Nor,
	/// TODO doc
	Equiv,
	/// TODO doc
	Invert,
	/// TODO doc
	OrReverse,
	/// TODO doc
	CopyInverted,
	/// TODO doc
	OrInverted,
	/// TODO doc
	Nand,
	/// TODO doc
	Set,
}

impl TryFrom<u8> for Function {
	type Error = &'static str;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::Clear),
			1 => Ok(Self::And),
			2 => Ok(Self::AndReverse),
			3 => Ok(Self::Copy),
			4 => Ok(Self::AndInverted),
			5 => Ok(Self::NoOp),
			6 => Ok(Self::Xor),
			7 => Ok(Self::Or),
			8 => Ok(Self::Nor),
			9 => Ok(Self::Equiv),
			10 => Ok(Self::Invert),
			11 => Ok(Self::OrReverse),
			12 => Ok(Self::CopyInverted),
			13 => Ok(Self::OrInverted),
			14 => Ok(Self::Nand),
			15 => Ok(Self::Set),

			_ => Err("Invalid GC function number"),
		}
	}
}

/// TODO doc
#[derive(Clone)]
pub enum LineStyle {
	/// TODO doc
	Solid,
	/// TODO doc
	OnOffDash,
	/// TODO doc
	DoubleDash,
}

impl TryFrom<u8> for LineStyle {
	type Error = &'static str;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::Solid),
			1 => Ok(Self::OnOffDash),
			2 => Ok(Self::DoubleDash),

			_ => Err("Invalid GC line style number"),
		}
	}
}

/// TODO doc
#[derive(Clone)]
pub enum CapStyle {
	/// TODO doc
	NotLast,
	/// TODO doc
	Butt,
	/// TODO doc
	Round,
	/// TODO doc
	Projecting,
}

impl TryFrom<u8> for CapStyle {
	type Error = &'static str;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::NotLast),
			1 => Ok(Self::Butt),
			2 => Ok(Self::Round),
			3 => Ok(Self::Projecting),

			_ => Err("Invalid GC cap style number"),
		}
	}
}

/// TODO doc
#[derive(Clone)]
pub enum JoinStyle {
	/// TODO doc
	Solid,
	/// TODO doc
	OnOffDash,
	/// TODO doc
	DoubleDash,
}

impl TryFrom<u8> for JoinStyle {
	type Error = &'static str;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::Solid),
			1 => Ok(Self::OnOffDash),
			2 => Ok(Self::DoubleDash),

			_ => Err("Invalid GC join style number"),
		}
	}
}

/// TODO doc
#[derive(Clone)]
pub enum FillStyle {
	/// TODO doc
	Solid,
	/// TODO doc
	Tiled,
	/// TODO doc
	Stippled,
	/// TODO doc
	OpaqueStippled,
}

impl TryFrom<u8> for FillStyle {
	type Error = &'static str;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::Solid),
			1 => Ok(Self::Tiled),
			2 => Ok(Self::Stippled),
			3 => Ok(Self::OpaqueStippled),

			_ => Err("Invalid GC fill style number"),
		}
	}
}

/// TODO doc
#[derive(Clone)]
pub enum FillRule {
	/// TODO doc
	EvenOdd,
	/// TODO doc
	Winding,
}

impl TryFrom<u8> for FillRule {
	type Error = &'static str;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::EvenOdd),
			1 => Ok(Self::Winding),

			_ => Err("Invalid GC fill rule number"),
		}
	}
}

/// TODO doc
#[derive(Clone)]
pub enum SubWindowMode {
	/// TODO doc
	ClipByChildren,
	/// TODO doc
	IncludeInferiors,
}

impl TryFrom<u8> for SubWindowMode {
	type Error = &'static str;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::ClipByChildren),
			1 => Ok(Self::IncludeInferiors),

			_ => Err("Invalid GC subwindow mode number"),
		}
	}
}

/// TODO doc
#[derive(Clone)]
pub enum ArcMode {
	/// TODO doc
	Chord,
	/// TODO doc
	PieSlice,
}

impl TryFrom<u8> for ArcMode {
	type Error = &'static str;

	fn try_from(v: u8) -> Result<Self, Self::Error> {
		match v {
			0 => Ok(Self::Chord),
			1 => Ok(Self::PieSlice),

			_ => Err("Invalid GC arc mode number"),
		}
	}
}

/// Value of a graphics context.
#[derive(Clone)]
pub struct Value {
	/// TODO doc
	pub function: Function,
	/// TODO doc
	pub plane_mask: u32,
	/// TODO doc
	pub foreground: u32,
	/// TODO doc
	pub background: u32,
	/// TODO doc
	pub line_width: u16,
	/// TODO doc
	pub line_style: LineStyle,
	/// TODO doc
	pub cap_style: CapStyle,
	/// TODO doc
	pub join_style: JoinStyle,
	/// TODO doc
	pub fill_style: FillStyle,
	/// TODO doc
	pub fill_rule: FillRule,
	/// TODO doc
	pub tile: u32,
	/// TODO doc
	pub stipple: u32,
	/// TODO doc
	pub tile_stipple_x_origin: i16,
	/// TODO doc
	pub tile_stipple_y_origin: i16,
	/// TODO doc
	pub font: u32,
	/// TODO doc
	pub subwindow_mode: SubWindowMode,
	/// TODO doc
	pub graphics_exposures: u8,
	/// TODO doc
	pub clip_x_origin: i16,
	/// TODO doc
	pub clip_y_origin: i16,
	/// TODO doc
	pub clip_mask: u32,
	/// TODO doc
	pub dash_offset: u16,
	/// TODO doc
	pub dashes: u8,
	/// TODO doc
	pub arc_mode: ArcMode,
}

/// Structure representing a graphics context.
#[derive(Clone)]
pub struct GC {
	/// The ID of the drawable.
	pub drawable: u32,

	/// TODO
	pub bitmask: u32,

	/// The list of values.
	pub values: Vec<Value>,
}