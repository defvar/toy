import * as React from "react";
import { Field } from "./types";
import Divider from "@mui/material/Divider";
import Typography from "@mui/material/Typography";
import { isObject } from "../../utils/types";
import { CommonField } from "./CommonField";
import { ValidationResult } from "./types";
import { getChildErrors, getValidationChild } from "./validation";
import { styled } from "@mui/material/styles";

export interface ObjectFieldsProps<T> {
    name: string;
    path: string;
    required: boolean;
    items: Field[];
    data: T;
    onChange: (data: T) => void;
    validation?: ValidationResult;
}

const FieldDiv = styled("div")(({ theme }) => ({
    marginBottom: theme.spacing(1),
}));

const FieldSetDiv = styled("div")(({ theme }) => ({
    padding: theme.spacing(0, 1),
}));

const ObjectHeaderDiv = styled("div")(({ theme }) => ({
    marginTop: theme.spacing(2),
    marginBottom: theme.spacing(2),
}));

export const ObjectFields = React.memo(
    <T extends {}>({
        name,
        path,
        items,
        data,
        onChange,
        validation,
    }: ObjectFieldsProps<T>) => {
        const getId = (name: string, path: string, sufix = ""): string => {
            return sufix ? `${path}-${name}-${sufix}` : `${path}-${name}`;
        };

        const handleFieldChange = React.useCallback(
            (name: string, value: unknown) => {
                const newData = {
                    ...data,
                    [name]: value,
                };
                onChange(newData);
            },
            [data, onChange]
        );

        const renderWidgets = (item: Field): JSX.Element => {
            if (!item.children) {
                let v = data;
                if (isObject(data)) {
                    v = data[item.name];
                }

                return (
                    <CommonField
                        key={getId(item.name, path, "Field")}
                        path={path}
                        field={item}
                        value={v}
                        onChange={handleFieldChange}
                        errors={getChildErrors(validation, item.name)}
                    />
                );
            } else {
                return (
                    <FieldSetDiv key={`${item.name}-FieldSet`}>
                        <ObjectHeaderDiv>
                            <Typography>{item.label}</Typography>
                            <Divider />
                        </ObjectHeaderDiv>
                        <FieldDiv key={getId(name, path, "Field")}>
                            <ObjectFields
                                key={getId(name, path, "ObjectFields")}
                                name={item.name}
                                path={path ? `${path}-${name}` : name}
                                required={item.required}
                                items={item.children}
                                data={data[item.name]}
                                onChange={(v) =>
                                    handleFieldChange(item.name, v)
                                }
                                validation={getValidationChild(
                                    validation,
                                    item.name
                                )}
                            />
                        </FieldDiv>
                    </FieldSetDiv>
                );
            }
        };

        return <>{items.map((x) => renderWidgets(x))}</>;
    }
);
