import * as React from "react";
import FormControl from "@mui/material/FormControl";
import MenuItem from "@mui/material/MenuItem";
import Select, { SelectChangeEvent } from "@mui/material/Select";
import InputLabel from "@mui/material/InputLabel";
import { WidgetProps } from "./WidgetProps";

export const SelectWidget = React.memo(
    ({
        id,
        label,
        value,
        required,
        onChange,
        isError,
        selectOptions,
    }: WidgetProps) => {
        const handleChange = (
            e: SelectChangeEvent<{ value: unknown }>,
            _: React.ReactNode
        ): void => {
            onChange(e.target.value as string);
        };

        return (
            <FormControl fullWidth required={required} error={isError}>
                <InputLabel>{label}</InputLabel>
                <Select
                    id={id}
                    error={isError}
                    label={label}
                    required={required}
                    value={value ? value : ""}
                    onChange={handleChange}
                >
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
    }
);
