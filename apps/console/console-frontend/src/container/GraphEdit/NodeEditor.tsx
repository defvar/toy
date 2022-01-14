import * as React from "react";
import { Actions, GraphEditState } from "../../modules/graphEdit";
import { Form, ValidationResult } from "../../components/form";
import { Stack, Box } from "@mui/material";

export interface NodeEditorPorps {
    state: GraphEditState;
    dispatch: React.Dispatch<Actions>;
}

function validate(v: any): ValidationResult {
    const r = { name: "root", errors: [] };
    return r;
}

export const NodeEditor = (props: NodeEditorPorps) => {
    const { dispatch, state } = props;

    const handleFormOnChange = React.useCallback((v) => {
        dispatch({
            type: "ChangeEditNode",
            payload: v,
        });
    }, []);

    return (
        <Stack spacing={2}>
            <Box>
                <Form
                    data={state.edit.config}
                    liveValidation={false}
                    onChange={handleFormOnChange}
                    validate={validate}
                    schema={state.edit.configSchema}
                />
            </Box>
        </Stack>
    );
};
