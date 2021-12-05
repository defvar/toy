import * as React from "react";
import { WidgetProps } from "./WidgetProps";
import FormControl from "@mui/material/FormControl";
import FormControlLabel from "@mui/material/FormControlLabel";
import Checkbox from "@mui/material/Checkbox";

export const CheckboxWidget = React.memo((props: WidgetProps) => {
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
});
