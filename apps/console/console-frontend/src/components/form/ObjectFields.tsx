import * as React from "react";
import { Widgets } from "./widgets";
import { Field } from "./types";
import { createStyles, makeStyles, Theme } from "@material-ui/core/styles";
import Divider from "@material-ui/core/Divider";
import Typography from "@material-ui/core/Typography";
import { isObject } from "../../utils/types";

export interface ObjectFieldsProps<T> {
    name: string;
    path: string;
    required: boolean;
    items: Field[];
    data: T;
    onChange: (data: T) => void;
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
        const renderWidget = (item: Field): JSX.Element => {
            const id = getId(item.name, path, "Widget");
            if (!(item.type in Widgets)) {
                return (
                    <div key={id}>{`id:${getId(
                        item.name,
                        path
                    )} no widget for type ${item.type}`}</div>
                );
            }
            if (item.type in Widgets) {
                const Widget = Widgets[item.type];
                let v = data;
                if (isObject(data)) {
                    v = data[item.name];
                }
                return (
                    <div
                        key={getId(item.name, path, "Field")}
                        className={classes.field}
                    >
                        <Widget
                            key={id}
                            id={id}
                            {...item}
                            value={v}
                            onChange={(v) => handleFieldChange(item.name, v)}
                        />
                    </div>
                );
            }
        };

        if (item) {
            if (!item.children) {
                return renderWidget(item);
            } else {
                return (
                    <div
                        key={`${item.name}-FieldSet`}
                        className={classes.fieldSet}
                    >
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
                                onChange={(v) =>
                                    handleFieldChange(item.name, v)
                                }
                            />
                        </div>
                    </div>
                );
            }
        }
        return null;
    };

    return <>{items.map((x) => renderWidgets(x))}</>;
};
