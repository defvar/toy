import * as React from "react";
import FormControl from "@material-ui/core/FormControl";
import MenuItem from "@material-ui/core/MenuItem";
import Select from "@material-ui/core/Select";
import InputLabel from "@material-ui/core/InputLabel";
import { WidgetProps } from "./WidgetProps";

export const SelectWidget = ({
    id,
    label,
    value,
    required,
    onChange,
    isError,
    selectOptions,
}: WidgetProps) => {
    const handleChange = (e: React.ChangeEvent<{ value: unknown }>): void => {
        onChange(e.target.value as string);
    };

    return (
        <FormControl fullWidth={true} required={required} error={isError}>
            <InputLabel>{label}</InputLabel>
            <Select
                id={id}
                error={isError}
                label={label}
                required={required}
                value={value}
                onChange={handleChange}
            >
                <MenuItem value="">
                    <em>None</em>
                </MenuItem>
                {selectOptions &&
                    selectOptions.map((v, i) => {
                        return (
                            <MenuItem key={i} value={v.value}>
                                {v.label}
                            </MenuItem>
                        );
                    })}
            </Select>
        </FormControl>
    );
};
