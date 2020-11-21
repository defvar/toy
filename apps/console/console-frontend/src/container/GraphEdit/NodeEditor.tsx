import * as React from "react";
import Button from "@material-ui/core/Button";
import Dialog from "@material-ui/core/Dialog";
import DialogActions from "@material-ui/core/DialogActions";
import DialogContent from "@material-ui/core/DialogContent";
import DialogTitle from "@material-ui/core/DialogTitle";
import SaveIcon from "@material-ui/icons/Save";
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
            >
                <DialogTitle id="form-dialog-title">
                    {state.edit.id}
                </DialogTitle>
                <DialogActions>
                    <Button variant="contained" onClick={handleClose}>
                        Cancel
                    </Button>
                    <Button
                        variant="contained"
                        onClick={handleSubmit}
                        color="primary"
                        startIcon={<SaveIcon />}
                    >
                        Save
                    </Button>
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
