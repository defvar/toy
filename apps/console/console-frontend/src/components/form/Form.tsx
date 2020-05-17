import * as React from "react";
import { Field } from "./types";
import { ObjectFields } from "./ObjectFields";
import { createStyles, makeStyles, Theme } from "@material-ui/core/styles";

export interface FormProps<T> {
    items: Field[];
    data: T;
    onChange: (data: T) => void;
}

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            backgroundColor: theme.palette.background.paper,
            margin: theme.spacing(4),
        },
    })
);

export const Form = <T extends {}>({ items, data, onChange }: FormProps<T>) => {
    const classes = useStyles();
    return (
        <form className={classes.root} noValidate autoComplete="off">
            <ObjectFields
                key={"Root-ObjectFields"}
                name={"root"}
                path={""}
                required={false}
                items={items}
                data={data}
                onChange={onChange}
            />
        </form>
    );
};
