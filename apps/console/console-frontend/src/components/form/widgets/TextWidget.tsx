import * as React from "react";
import { WidgetProps } from "./WidgetProps";
import FormControl from "@material-ui/core/FormControl";
import TextField from "@material-ui/core/TextField";

export const TextWidget = React.memo((props: WidgetProps) => {
    const { id, label, value, required, onChange, isError } = props;
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>): void => {
        onChange(e.target.value);
    };
    return (
        <FormControl fullWidth={true} required={required} error={isError}>
            <TextField
                id={id}
                error={isError}
                label={label}
                required={required}
                value={value ? value : ""}
                onChange={handleChange}
            />
        </FormControl>
    );
});
