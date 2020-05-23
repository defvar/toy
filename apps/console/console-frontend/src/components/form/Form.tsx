import * as React from "react";
import { Field, ValidationResult } from "./types";
import { ObjectFields } from "./ObjectFields";
import { createStyles, makeStyles, Theme } from "@material-ui/core/styles";

export interface FormProps<T> {
    items: Field[];
    data: T;
    onChange: (data: T) => void;
    validate: (data: T) => ValidationResult;
    liveValidation?: boolean;
}

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            backgroundColor: theme.palette.background.paper,
            margin: theme.spacing(4),
        },
    })
);

export const Form = <T extends {}>({
    items,
    data,
    onChange,
    validate,
    liveValidation,
}: FormProps<T>) => {
    const classes = useStyles();

    const [validationState, setValidationState] = React.useState(() => ({
        name: "root",
        errors: [],
    }));

    const handleOnChange = (data: T) => {
        onChange(data);
        if (liveValidation) {
            const r = validate(data);
            setValidationState(r);
        }
    };

    return (
        <form className={classes.root} noValidate autoComplete="off">
            <ObjectFields
                key={"Root-ObjectFields"}
                name={"root"}
                path={""}
                required={false}
                items={items}
                data={data}
                onChange={handleOnChange}
                validation={validationState}
            />
        </form>
    );
};
