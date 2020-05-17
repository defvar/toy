import * as React from "react";
import { WidgetProps } from "./WidgetProps";
import FormControl from "@material-ui/core/FormControl";
import TextField from "@material-ui/core/TextField";

export const TextWidget = (props: WidgetProps) => {
    const { id, label, value, required, onChange } = props;
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>): void => {
        onChange(e.target.value);
    };
    return (
        <FormControl fullWidth={true} required={required}>
            <TextField
                id={id}
                label={label}
                required={required}
                value={value ? value : ""}
                onChange={handleChange}
            />
        </FormControl>
    );
};
