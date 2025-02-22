use proc_macro::TokenStream;
use std::fmt::{write, Write};
use ringhopper_definitions::{Bitfield, Enum, FieldCount, NamedObject, ObjectType, ParsedDefinitions, Struct, StructFieldType};

#[proc_macro]
pub fn tag_definitions(_: TokenStream) -> TokenStream {
    let definitions = ringhopper_definitions::load_all_definitions();
    let mut data = String::with_capacity(4 * 1024 * 1024);

    for (_, object) in &definitions.objects {
        match object {
            NamedObject::Struct(s) => write_struct(s, &definitions, &mut data),
            NamedObject::Enum(e) => write_enum(e, &definitions, &mut data),
            NamedObject::Bitfield(b) => write_bitfield(b, &definitions, &mut data)
        }
    }

    for (_, group) in &definitions.groups {
        let struct_name = &group.struct_name;
        data.write_fmt(format_args!(r#"
impl TagGroupStruct for {struct_name} {{
    fn get_tag_group() -> primitives::tag_group::TagGroup {{
        primitives::tag_group::TagGroup::{struct_name}
    }}
}}"#)).unwrap()
    }

    data.parse().expect("failed to parse tag definitions ;-;")
}

fn write_struct(struct_data: &Struct, definitions: &ParsedDefinitions, output: &mut String) {
    let name = &struct_data.name;
    let size = struct_data.size;
    let mut members = String::with_capacity(1024 * 1024);

    for (index, i) in struct_data.fields.iter().enumerate() {
        match &i.field_type {
            StructFieldType::Padding(p) => {
                write(&mut members, format_args!("    pub _padding_{index}: [u8; {p}],\n")).expect(";-;");
            },
            StructFieldType::EditorSection(_) => {},
            StructFieldType::Object(o) => {
                let object_type = match o {
                    ObjectType::NamedObject(n) => {
                        match definitions.objects.get(n) {
                            Some(NamedObject::Enum(_)) => format!("primitives::EnumWrapper<{n}>"),
                            _ => n.to_owned()
                        }
                    },
                    ObjectType::Float => "f32".to_owned(),
                    ObjectType::Address => "primitives::data::Address".to_owned(),
                    ObjectType::Reflexive(what) => format!("primitives::data::Reflexive<{what}>"),
                    ObjectType::TagReference(_) => format!("primitives::data::TagReference"),
                    ObjectType::TagGroup => "primitives::data::TagGroupFourCC".to_owned(),
                    ObjectType::Data => "primitives::data::Data".to_owned(),
                    ObjectType::FileData => "primitives::data::FileData".to_owned(),
                    ObjectType::BSPVertexData => "primitives::data::Data".to_owned(),
                    ObjectType::U8 => "u8".to_owned(),
                    ObjectType::U16 => "u16".to_owned(),
                    ObjectType::U32 => "u32".to_owned(),
                    ObjectType::I8 => "i8".to_owned(),
                    ObjectType::I16 => "i16".to_owned(),
                    ObjectType::I32 => "i32".to_owned(),
                    ObjectType::TagID => "primitives::data::TagID".to_owned(),
                    ObjectType::ID => "primitives::data::ID".to_owned(),
                    ObjectType::Index => "primitives::data::Index".to_owned(),
                    ObjectType::Angle => "primitives::vector::Angle".to_owned(),
                    ObjectType::Vector2D => "primitives::vector::Vector2D".to_owned(),
                    ObjectType::Vector3D => "primitives::vector::Vector3D".to_owned(),
                    ObjectType::CompressedVector2D => "primitives::vector::CompressedVector2D".to_owned(),
                    ObjectType::CompressedVector3D => "primitives::vector::CompressedVector3D".to_owned(),
                    ObjectType::CompressedFloat => "primitives::vector::CompressedFloat".to_owned(),
                    ObjectType::Vector2DInt => "primitives::vector::Vector2DInt".to_owned(),
                    ObjectType::Plane2D => "primitives::vector::Plane2D".to_owned(),
                    ObjectType::Plane3D => "primitives::vector::Plane3D".to_owned(),
                    ObjectType::Euler2D => "primitives::vector::Euler2D".to_owned(),
                    ObjectType::Euler3D => "primitives::vector::Euler3D".to_owned(),
                    ObjectType::Rectangle => "primitives::vector::Rectangle".to_owned(),
                    ObjectType::Quaternion => "primitives::vector::Quaternion".to_owned(),
                    ObjectType::Matrix3x3 => "primitives::vector::Matrix3x3".to_owned(),
                    ObjectType::ColorRGBFloat => "primitives::color::ColorRGB".to_owned(),
                    ObjectType::ColorARGBFloat => "primitives::color::ColorARGB".to_owned(),
                    ObjectType::ColorARGBInt => "primitives::color::Pixel32".to_owned(),
                    ObjectType::String32 => "primitives::string::String32".to_owned(),
                    ObjectType::ScenarioScriptNodeValue => "primitives::data::ScenarioScriptNodeValue".to_owned(),
                    ObjectType::UTF16String => "primitives::data::Data".to_owned(),
                };

                let object_type = match i.count {
                    FieldCount::One => object_type,
                    FieldCount::Array(l) => format!("[{object_type}; {l}]"),
                    FieldCount::Bounds => format!("primitives::Bounds<{object_type}>")
                };

                let struct_field_name = rustify_string(&i.name);
                write(&mut members, format_args!("    pub {struct_field_name}: {object_type},\n")).expect(";-;");
            }
        }
    }

    write(output, format_args!(r#"
#[repr(C, packed)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct {name} {{
{members}}}
const _: () = assert!(size_of::<{name}>() == {size}, "bad size {name}; should be {size} bytes");
impl NamedTagStruct for {name} {{
    fn name() -> &'static str {{
        "{name}"
    }}
}}
"#)).expect(";-;");
}

fn write_enum(enum_data: &Enum, _definitions: &ParsedDefinitions, output: &mut String) {
    let name = &enum_data.name;
    let mut members = String::with_capacity(1024 * 1024);
    let mut members_names = String::with_capacity(1024 * 1024);
    for i in &enum_data.options {
        let name = rustify_string_pascal_case(&i.name);
        write(&mut members, format_args!("    {name} = {},\n", i.value)).expect(";-;");
        write(&mut members_names, format_args!("    Self::{name} => \"{}\",\n", i.name)).expect(";-;");
    }

    write(output, format_args!(r#"
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, TryFromPrimitive)]
pub enum {name} {{
{members}}}
impl NamedTagStruct for {name} {{
    fn name() -> &'static str {{
        "{name}"
    }}
}}
impl {name} {{
    fn as_str(self) -> &'static str {{
        match self {{
            {members_names}
        }}
    }}
}}
impl core::fmt::Display for {name} {{
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
        fmt.write_str(self.as_str())
    }}
}}"#)).expect(";-;");
}

fn write_bitfield(bitfield_data: &Bitfield, _definitions: &ParsedDefinitions, output: &mut String) {
    let width = bitfield_data.width;
    let name = &bitfield_data.name;

    let mut members = String::with_capacity(1024 * 1024);
    for i in &bitfield_data.fields {
        let name = rustify_string_pascal_case(&i.name);
        write(&mut members, format_args!("    {name} = 0x{:08X},\n", i.value)).expect(";-;")
    }

    write(output, format_args!(r#"
#[repr(u{width})]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum {name}Fields {{
{members}}}"#)).expect(";-;");

    write(output, format_args!(r#"
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct {name}(pub u{width});

impl NamedTagStruct for {name} {{
    fn name() -> &'static str {{
        "{name}"
    }}
}}

impl {name} {{
    pub const fn is_set(self, field: {name}Fields) -> bool {{
        self.0 & (field as u{width}) != 0
    }}
    pub const fn set(&mut self, field: {name}Fields) {{
        self.0 = self.0 | (field as u{width});
    }}
    pub const fn unset(&mut self, field: {name}Fields) {{
        self.0 = self.0 & !(field as u{width});
    }}
}}
"#)).expect(";-;");
}

fn rustify_string(string: &str) -> String {
    let name = string
        .replace(" ", "_")
        .replace("-", "_")
        .replace("(", "")
        .replace(")", "")
        .replace("[", "")
        .replace("]", "")
        .replace("'", "");

    match name.as_str() {
        "type" => "r#type".to_owned(),
        "loop" => "r#loop".to_owned(),
        "self" => "r#self".to_owned(),
        "super" => "r#super".to_owned(),
        n if n.chars().next().unwrap().is_numeric() => format!("_{name}"),
        _ => name
    }
}

fn rustify_string_pascal_case(string: &str) -> String {
    let mut string_modified = string.to_owned();
    // SAFETY: Trust me bro
    let bytes = unsafe { string_modified.as_bytes_mut() };
    let mut word_boundary = true;
    for (index, c) in string.char_indices() {
        if word_boundary {
            bytes[index] = c.to_ascii_uppercase() as u8;
        }
        word_boundary = c == ' ';
    }
    string_modified = string_modified.replace(" ", "");
    rustify_string(&string_modified)
}
