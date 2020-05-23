import * as React from "react";
import { WidgetProps } from "./WidgetProps";
import FormControl from "@material-ui/core/FormControl";
import FormControlLabel from "@material-ui/core/FormControlLabel";
import Checkbox from "@material-ui/core/Checkbox";

export const CheckboxWidget = (props: WidgetProps) => {
    const { id, label, value, required, onChange, isError } = props;
    const handleChange = (
        _e: React.ChangeEvent<HTMLInputElement>,
        checked: boolean
    ): void => {
        onChange(checked);
    };
    return (
        <FormControl fullWidth={true} required={required} error={isError}>
            <FormControlLabel
                control={
                    <Checkbox
                        id={id}
                        required={required}
                        checked={value ? true : false}
                        onChange={handleChange}
                    />
                }
                label={label}
            />
        </FormControl>
    );
};
