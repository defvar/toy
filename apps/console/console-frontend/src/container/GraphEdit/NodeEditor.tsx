import * as React from "react";
import Button from "@mui/material/Button";
import Dialog from "@mui/material/Dialog";
import DialogActions from "@mui/material/DialogActions";
import DialogContent from "@mui/material/DialogContent";
import DialogTitle from "@mui/material/DialogTitle";
import SaveIcon from "@mui/icons-material/Save";
import { Actions, GraphEditState } from "../../modules/graphEdit";
import { Form, ValidationResult } from "../../components/form";

export interface NodeEditorPorps {
    state: GraphEditState;
    dispatch: React.Dispatch<Actions>;
    open: boolean;
}

function validate(v: any): ValidationResult {
    const r = { name: "root", errors: [] };
    return r;
}

export const NodeEditor = (props: NodeEditorPorps) => {
    const { open, dispatch, state } = props;
    const handleClose = React.useCallback(() => {
        dispatch({
            type: "CancelEditNode",
        });
    }, []);

    const handleSubmit = React.useCallback(() => {
        dispatch({
            type: "SubmitEditNode",
        });
    }, []);

    const handleFormOnChange = React.useCallback((v) => {
        dispatch({
            type: "ChangeEditNode",
            payload: v,
        });
    }, []);

    return (
        <>
            <Dialog
                open={open}
                onClose={handleClose}
                aria-labelledby="form-dialog-title"
                maxWidth={"md"}
                fullWidth={true}
                sx={{ zIndex: 20000 }}
            >
                <DialogTitle id="form-dialog-title">
                    {state.edit.id}
                </DialogTitle>
                <DialogActions>
                    <Button onClick={handleSubmit} startIcon={<SaveIcon />}>
                        Save
                    </Button>
                    <Button onClick={handleClose}>Cancel</Button>
                </DialogActions>
                <DialogContent dividers={true}>
                    <Form
                        data={state.edit.config}
                        liveValidation={false}
                        onChange={handleFormOnChange}
                        validate={validate}
                        schema={state.edit.configSchema}
                    />
                </DialogContent>
            </Dialog>
        </>
    );
};
