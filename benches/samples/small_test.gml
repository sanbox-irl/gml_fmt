enum EImGui_ComboFlags
{
    PopupAlignLeft          = 1 << 0,   // Align the popup toward the left by default
    HeightSmall             = 1 << 1,   // Max ~4 items visible. Tip: If you want your combo popup to be a specific size you can use SetNextWindowSizeConstraints() prior to calling BeginCombo()
    HeightRegular           = 1 << 2,   // Max ~8 items visible (default)
    HeightLarge             = 1 << 3,   // Max ~20 items visible
    HeightLargest           = 1 << 4,   // As many fitting items as possible
    HeightMask_             = EImGui_ComboFlags.HeightSmall | EImGui_ComboFlags.HeightRegular | EImGui_ComboFlags.HeightLarge | EImGui_ComboFlags.HeightLargest
};
