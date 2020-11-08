import * as React from "react";
import { ValidationResult } from "./types";
import { parse } from "./util";
import { ObjectFields } from "./ObjectFields";
import { createStyles, makeStyles, Theme } from "@material-ui/core/styles";
import { JsonSchema } from "../../modules/common";

export interface FormProps<T> {
    schema: JsonSchema;
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
    schema,
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

    const items = React.useMemo(() => {
        return parse(schema);
    }, [schema]);

    const handleOnChange = (d: T) => {
        onChange(d);
        if (liveValidation) {
            const r = validate(d);
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
