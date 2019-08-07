enum EInputConstants{//Starts at high negative number to not interfere with other input constants
//P - Positive axis (Axis is regular coordinates with 0;0 being bottom left)
//N - Negative axis
//Note that return is always positive
    GP_AXISLV_P = -100, //down
    GP_AXISLV_N, //up
    GP_AXISLH_P,
    GP_AXISLH_N,
    GP_AXISRV_P, //down
    GP_AXISRV_N, //up
    GP_AXISRH_P,
    GP_AXISRH_N,
    SCROLL_DOWN,
    SCROLL_UP,
    ANY,
    NONE
}