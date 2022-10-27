//! This module implements opcodes.

/// Enumeration of opcodes (excluding extentions).
pub enum Opcode {
	CreateWindow,
	ChangeWindowAttributes,
	GetWindowAttributes,
	DestroyWindow,
	DestroySubwindows,
	ChangeSaveSet,
	ReparentWindow,
	MapWindow,
	MapSubwindows,
	UnmapWindow,
	UnmapSubwindows,
	ConfigureWindow,
	CirculateWindow,
	GetGeometry,
	QueryTree,
	InternAtom,
	GetAtomName,
	ChangeProperty,
	DeleteProperty,
	GetProperty,
	ListProperties,
	SetSelectionOwner,
	GetSelectionOwner,
	ConvertSelection,
	SendEvent,
	GrabPointer,
	UngrabPointer,
	GrabButton,
	UngrabButton,
	ChangeActivePointerGrab,
	GrabKeyboard,
	UngrabKeyboard,
	GrabKey,
	UngrabKey,
	AllowEvents,
	GrabServer,
	UngrabServer,
	QueryPointer,
	GetMotionEvents,
	TranslateCoordinates,
	WarpPointer,
	SetInputFocus,
	GetInputFocus,
	QueryKeymap,
	OpenFont,
	CloseFont,
	QueryFont,
	QueryTextExtents,
	ListFonts,
	ListFontsWithInfo,
	SetFontPath,
	GetFontPath,
	CreatePixmap,
	FreePixmap,
	CreateGC,
	ChangeGC,
	CopyGC,
	SetDashes,
	SetClipRectangles,
	FreeGC,
	ClearArea,
	CopyArea,
	CopyPlane,
	PolyPoint,
	PolyLine,
	PolySegment,
	PolyRectangle,
	PolyArc,
	FillPoly,
	PolyFillRectangle,
	PolyFillArc,
	PutImage,
	GetImage,
	PolyText8,
	PolyText16,
	ImageText8,
	ImageText16,
	CreateColormap,
	FreeColormap,
	CopyColormapAndFree,
	InstallColormap,
	UninstallColormap,
	ListInstalledColormaps,
	AllocColor,
	AllocNamedColor,
	AllocColorCells,
	AllocColorPlanes,
	FreeColors,
	StoreColors,
	StoreNamedColor,
	QueryColors,
	LookupColor,
	CreateCursor,
	CreateGlyphCursor,
	FreeCursor,
	RecolorCursor,
	QueryBestSize,
	QueryExtension,
	ListExtensions,
	ChangeKeyboardMapping,
	GetKeyboardMapping,
	ChangeKeyboardControl,
	GetKeyboardControl,
	Bell,
	ChangePointerControl,
	GetPointerControl,
	SetScreenSaver,
	GetScreenSaver,
	ChangeHosts,
	ListHosts,
	SetAccessControl,
	SetCloseDownMode,
	KillClient,
	RotateProperties,
	ForceScreenSaver,
	SetPointerMapping,
	GetPointerMapping,
	SetModifierMapping,
	GetModifierMapping,
	NoOperation,
}

impl Opcode {
	/// Returns the opcode with the given ID `id`.
	/// If the opcode doesn't exist, the function returns None.
	pub fn from_id(id: u8) -> Option<Self> {
		match id {
			1 => Some(Self::CreateWindow),
			2 => Some(Self::ChangeWindowAttributes),
			3 => Some(Self::GetWindowAttributes),
			4 => Some(Self::DestroyWindow),
			5 => Some(Self::DestroySubwindows),
			6 => Some(Self::ChangeSaveSet),
			7 => Some(Self::ReparentWindow),
			8 => Some(Self::MapWindow),
			9 => Some(Self::MapSubwindows),
			10 => Some(Self::UnmapWindow),
			11 => Some(Self::UnmapSubwindows),
			12 => Some(Self::ConfigureWindow),
			13 => Some(Self::CirculateWindow),
			14 => Some(Self::GetGeometry),
			15 => Some(Self::QueryTree),
			16 => Some(Self::InternAtom),
			17 => Some(Self::GetAtomName),
			18 => Some(Self::ChangeProperty),
			19 => Some(Self::DeleteProperty),
			20 => Some(Self::GetProperty),
			21 => Some(Self::ListProperties),
			22 => Some(Self::SetSelectionOwner),
			23 => Some(Self::GetSelectionOwner),
			24 => Some(Self::ConvertSelection),
			25 => Some(Self::SendEvent),
			26 => Some(Self::GrabPointer),
			27 => Some(Self::UngrabPointer),
			28 => Some(Self::GrabButton),
			29 => Some(Self::UngrabButton),
			30 => Some(Self::ChangeActivePointerGrab),
			31 => Some(Self::GrabKeyboard),
			32 => Some(Self::UngrabKeyboard),
			33 => Some(Self::GrabKey),
			34 => Some(Self::UngrabKey),
			35 => Some(Self::AllowEvents),
			36 => Some(Self::GrabServer),
			37 => Some(Self::UngrabServer),
			38 => Some(Self::QueryPointer),
			39 => Some(Self::GetMotionEvents),
			40 => Some(Self::TranslateCoordinates),
			41 => Some(Self::WarpPointer),
			42 => Some(Self::SetInputFocus),
			43 => Some(Self::GetInputFocus),
			44 => Some(Self::QueryKeymap),
			45 => Some(Self::OpenFont),
			46 => Some(Self::CloseFont),
			47 => Some(Self::QueryFont),
			48 => Some(Self::QueryTextExtents),
			49 => Some(Self::ListFonts),
			50 => Some(Self::ListFontsWithInfo),
			51 => Some(Self::SetFontPath),
			52 => Some(Self::GetFontPath),
			53 => Some(Self::CreatePixmap),
			54 => Some(Self::FreePixmap),
			55 => Some(Self::CreateGC),
			56 => Some(Self::ChangeGC),
			57 => Some(Self::CopyGC),
			58 => Some(Self::SetDashes),
			59 => Some(Self::SetClipRectangles),
			60 => Some(Self::FreeGC),
			61 => Some(Self::ClearArea),
			62 => Some(Self::CopyArea),
			63 => Some(Self::CopyPlane),
			64 => Some(Self::PolyPoint),
			65 => Some(Self::PolyLine),
			66 => Some(Self::PolySegment),
			67 => Some(Self::PolyRectangle),
			68 => Some(Self::PolyArc),
			69 => Some(Self::FillPoly),
			70 => Some(Self::PolyFillRectangle),
			71 => Some(Self::PolyFillArc),
			72 => Some(Self::PutImage),
			73 => Some(Self::GetImage),
			74 => Some(Self::PolyText8),
			75 => Some(Self::PolyText16),
			76 => Some(Self::ImageText8),
			77 => Some(Self::ImageText16),
			78 => Some(Self::CreateColormap),
			79 => Some(Self::FreeColormap),
			80 => Some(Self::CopyColormapAndFree),
			81 => Some(Self::InstallColormap),
			82 => Some(Self::UninstallColormap),
			83 => Some(Self::ListInstalledColormaps),
			84 => Some(Self::AllocColor),
			85 => Some(Self::AllocNamedColor),
			86 => Some(Self::AllocColorCells),
			87 => Some(Self::AllocColorPlanes),
			88 => Some(Self::FreeColors),
			89 => Some(Self::StoreColors),
			90 => Some(Self::StoreNamedColor),
			91 => Some(Self::QueryColors),
			92 => Some(Self::LookupColor),
			93 => Some(Self::CreateCursor),
			94 => Some(Self::CreateGlyphCursor),
			95 => Some(Self::FreeCursor),
			96 => Some(Self::RecolorCursor),
			97 => Some(Self::QueryBestSize),
			98 => Some(Self::QueryExtension),
			99 => Some(Self::ListExtensions),
			100 => Some(Self::ChangeKeyboardMapping),
			101 => Some(Self::GetKeyboardMapping),
			102 => Some(Self::ChangeKeyboardControl),
			103 => Some(Self::GetKeyboardControl),
			104 => Some(Self::Bell),
			105 => Some(Self::ChangePointerControl),
			106 => Some(Self::GetPointerControl),
			107 => Some(Self::SetScreenSaver),
			108 => Some(Self::GetScreenSaver),
			109 => Some(Self::ChangeHosts),
			110 => Some(Self::ListHosts),
			111 => Some(Self::SetAccessControl),
			112 => Some(Self::SetCloseDownMode),
			113 => Some(Self::KillClient),
			114 => Some(Self::RotateProperties),
			115 => Some(Self::ForceScreenSaver),
			116 => Some(Self::SetPointerMapping),
			117 => Some(Self::GetPointerMapping),
			118 => Some(Self::SetModifierMapping),
			119 => Some(Self::GetModifierMapping),
			127 => Some(Self::NoOperation),

			_ => None,
		}
	}
}