import * as React from "react";
import { WidgetProps } from "./WidgetProps";
import FormControl from "@mui/material/FormControl";
import Input from "@mui/material/Input";
import InputLabel from "@mui/material/InputLabel";

export const NumberWidget = React.memo((props: WidgetProps) => {
    const { id, label, value, required, onChange, isError } = props;
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>): void => {
        onChange(e.target.value);
    };
    return (
        <FormControl fullWidth={true} required={required} error={isError}>
            <InputLabel>{label}</InputLabel>
            <Input
                id={id}
                error={isError}
                type="number"
                required={required}
                value={value ? value : ""}
                onChange={handleChange}
            />
        </FormControl>
    );
});
