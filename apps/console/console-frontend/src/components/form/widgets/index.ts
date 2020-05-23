export { WidgetProps } from "./WidgetProps";
export { TextWidget } from "./TextWidget";
export { NumberWidget } from "./NumberWidget";
export { SelectWidget } from "./SelectWidget";

import { TextWidget } from "./TextWidget";
import { NumberWidget } from "./NumberWidget";
import { CheckboxWidget } from "./CheckboxWidget";
import { SelectWidget } from "./SelectWidget";

export const Widgets = {
    number: NumberWidget,
    string: TextWidget,
    boolean: CheckboxWidget,
    enum: SelectWidget,
};
