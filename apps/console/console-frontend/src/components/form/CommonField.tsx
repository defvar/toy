import * as React from "react";
import { Widgets } from "./widgets";
import { Field, FieldError } from "./types";
import FormHelperText from "@mui/material/FormHelperText";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import { styled } from "@mui/material/styles";

export interface CommonFieldProps<T> {
    path: string;
    field: Field;
    value: T;
    onChange: (name: string, value: T) => void;
    errors: FieldError[];
}

const Area = styled("div")(({ theme }) => ({
    marginBottom: theme.spacing(1),
}));

const getId = (name: string, path: string, sufix = ""): string => {
    return sufix ? `${path}-${name}-${sufix}` : `${path}-${name}`;
};

export const CommonField = React.memo(
    <T extends {}>({
        path,
        field,
        value,
        onChange,
        errors,
    }: CommonFieldProps<T>) => {
        const isError = errors.length > 0;

        const handleOnChange = React.useCallback(
            (value: T) => {
                onChange(field.name, value);
            },
            [field, onChange]
        );

        const id = getId(field.name, path, "Widget");
        if (!(field.type in Widgets)) {
            return <div>{`id:${id} no widget for type ${field.type}`}</div>;
        }
        if (field.type in Widgets) {
            const Widget = Widgets[field.type];
            return (
                <Area>
                    <Widget
                        key={id}
                        id={id}
                        label={field.label}
                        required={field.required}
                        selectOptions={field.selectOptions}
                        value={value}
                        isError={isError}
                        onChange={handleOnChange}
                    />
                    {errors.length > 0 && (
                        <List dense={true}>
                            {errors.map((error, i: number) => {
                                return (
                                    <ListItem key={i}>
                                        <FormHelperText id={id}>
                                            - {error.message}
                                        </FormHelperText>
                                    </ListItem>
                                );
                            })}
                        </List>
                    )}
                </Area>
            );
        }
    }
);
