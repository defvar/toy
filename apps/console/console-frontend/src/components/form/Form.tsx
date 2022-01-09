import * as React from "react";
import { ValidationResult } from "./types";
import { parse } from "./util";
import { ObjectFields } from "./ObjectFields";
import { JsonSchema } from "../../modules/common";
import { Box } from "@mui/material";

export interface FormProps<T> {
    schema: JsonSchema;
    data: T;
    onChange: (data: T) => void;
    validate: (data: T) => ValidationResult;
    liveValidation?: boolean;
}

export const Form = <T extends {}>({
    schema,
    data,
    onChange,
    validate,
    liveValidation,
}: FormProps<T>) => {
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
        <Box
            component="form"
            sx={{
                m: 1,
                width: "50ch",
                "& .MuiTextField-root": {
                    m: 1,
                    width: "50ch",
                    minWidth: "25ch",
                },
            }}
            noValidate
            autoComplete="off"
        >
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
        </Box>
    );
};
