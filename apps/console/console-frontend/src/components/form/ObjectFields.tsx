import * as React from "react";
import { Field } from "./types";
import { createStyles, makeStyles, Theme } from "@material-ui/core/styles";
import Divider from "@material-ui/core/Divider";
import Typography from "@material-ui/core/Typography";
import { isObject } from "../../utils/types";
import { CommonField } from "./CommonField";
import { ValidationResult } from "./types";
import { getChildErrors, getValidationChild } from "./validation";

export interface ObjectFieldsProps<T> {
    name: string;
    path: string;
    required: boolean;
    items: Field[];
    data: T;
    onChange: (data: T) => void;
    validation?: ValidationResult;
}

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        field: {
            marginBottom: theme.spacing(1),
        },
        fieldSet: {
            padding: theme.spacing(0, 1),
        },
        objectHeader: {
            marginTop: theme.spacing(2),
            marginBottom: theme.spacing(2),
        },
    })
);

export const ObjectFields = <T extends {}>({
    name,
    path,
    items,
    data,
    onChange,
    validation,
}: ObjectFieldsProps<T>) => {
    const classes = useStyles();

    const getId = (name: string, path: string, sufix = ""): string => {
        return sufix ? `${path}-${name}-${sufix}` : `${path}-${name}`;
    };

    const handleFieldChange = (name: string, value: unknown) => {
        const newData = {
            ...data,
            [name]: value,
        };
        onChange(newData);
    };

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
                <div key={`${item.name}-FieldSet`} className={classes.fieldSet}>
                    <div className={classes.objectHeader}>
                        <Typography>{item.label}</Typography>
                        <Divider />
                    </div>
                    <div
                        key={getId(name, path, "Field")}
                        className={classes.field}
                    >
                        <ObjectFields
                            key={getId(name, path, "ObjectFields")}
                            name={item.name}
                            path={path ? `${path}-${name}` : name}
                            required={item.required}
                            items={item.children}
                            data={data[item.name]}
                            onChange={(v) => handleFieldChange(item.name, v)}
                            validation={getValidationChild(
                                validation,
                                item.name
                            )}
                        />
                    </div>
                </div>
            );
        }
    };

    return <>{items.map((x) => renderWidgets(x))}</>;
};
