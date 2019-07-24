fn main() {
    gml_fmt::run(
        "/// @function GetSelectionWater()
return GetSelectionData(SelectionData.Type) == SelectionType.Land;",
    );
}
