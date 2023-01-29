import * as React from "react";
import { Box, Modal, Stack, Alert, Snackbar, Typography } from "@mui/material";
import {
    DataGrid,
    GridColumns,
    GridRowParams,
    GridRowsProp,
    MuiEvent,
    GridRowId,
    GridRowModel,
    GridEventListener,
    GridRowModes,
    GridRowModesModel,
    GridActionsCellItem,
} from "@mui/x-data-grid";
import AddIcon from "@mui/icons-material/Add";
import EditIcon from "@mui/icons-material/Edit";
import DeleteIcon from "@mui/icons-material/DeleteOutlined";
import SaveIcon from "@mui/icons-material/Save";
import CancelIcon from "@mui/icons-material/Close";
import { Role, RbacClient, ErrorMessage, useFetch } from "../../modules/api";

const columns = (
    rowModesModel: GridRowModesModel,
    handleSaveClick: (id: GridRowId) => () => void,
    handleCancelClick: (id: GridRowId) => () => void,
    handleEditClick: (id: GridRowId) => () => void,
    handleDeleteClick: (id: GridRowId) => () => void
): GridColumns => {
    return [
        { field: "resource", headerName: "resource", width: 150 },
        {
            field: "get",
            type: "boolean",
            headerName: "GET",
            width: 150,
            editable: true,
        },
        {
            field: "post",
            type: "boolean",
            headerName: "POST",
            width: 150,
            editable: true,
        },
        {
            field: "put",
            type: "boolean",
            headerName: "PUT",
            width: 150,
            editable: true,
        },
        {
            field: "delete",
            type: "boolean",
            headerName: "DELETE",
            width: 150,
            editable: true,
        },
        {
            field: "actions",
            type: "actions",
            headerName: "Actions",
            width: 100,
            cellClassName: "actions",
            getActions: ({ id }) => {
                const isInEditMode =
                    rowModesModel[id]?.mode === GridRowModes.Edit;

                if (isInEditMode) {
                    return [
                        <GridActionsCellItem
                            icon={<SaveIcon />}
                            label="Save"
                            onClick={handleSaveClick(id)}
                        />,
                        <GridActionsCellItem
                            icon={<CancelIcon />}
                            label="Cancel"
                            className="textPrimary"
                            onClick={handleCancelClick(id)}
                            color="inherit"
                        />,
                    ];
                }

                return [
                    <GridActionsCellItem
                        icon={<EditIcon />}
                        label="Edit"
                        className="textPrimary"
                        onClick={handleEditClick(id)}
                        color="inherit"
                    />,
                    <GridActionsCellItem
                        icon={<DeleteIcon />}
                        label="Delete"
                        onClick={handleDeleteClick(id)}
                        color="inherit"
                    />,
                ];
            },
        },
    ];
};

interface Row {
    id: string;
    resource: string;
    get: boolean;
    post: boolean;
    put: boolean;
    delete: boolean;
    isDirty: boolean;
}

function toItems(role: Role): Row[] {
    if (!role) return [];
    if (!role.rules) return [];

    let r = {};
    for (const rule of role.rules) {
        for (const resource of rule.resources) {
            if (!r[resource]) {
                r[resource] = { id: resource, resource };
            }
            for (const verb of rule.verbs) {
                if (verb != "*") {
                    r[resource][verb.toLowerCase()] = true;
                } else {
                    r[resource] = {
                        id: resource,
                        resource,
                        get: true,
                        post: true,
                        put: true,
                        delete: true,
                    };
                }
            }
        }
    }

    return Object.values(r);
}

function toModel(prev: Role, items: Row[]): Role {
    if (!items) return null;

    const r = {
        name: prev.name,
        note: prev.note,
        rules: [],
    } as Role;

    let rules = [];
    for (const item of items) {
        let ruleItem = { resources: [], verbs: [] };
        ruleItem.resources.push(item.resource);
        if (item.get) {
            ruleItem.verbs.push("GET");
        }
        if (item.post) {
            ruleItem.verbs.push("POST");
        }
        if (item.put) {
            ruleItem.verbs.push("PUT");
        }
        if (item.delete) {
            ruleItem.verbs.push("DELETE");
        }
        rules.push(ruleItem);
    }

    r.rules = rules;
    return r;
}

const style = {
    position: "relative" as "relative",
    top: "50%",
    left: "50%",
    transform: "translate(-50%, -50%)",
    height: "500px",
    width: "100%",
    maxWidth: "1000px",
    bgcolor: "background.paper",
    border: "2px solid #000",
    boxShadow: 24,
    p: 4,
    display: "flex",
};

export interface RuleListProps {
    name: string;
    open: boolean;
    onClose: () => void;
}

const PutResult = ({ resource, snackOpen, onSnackClose, onSnackOpen }) => {
    const response = resource.read();
    const r = response && response.isSuccess() ? response.value : null;
    if (r) {
        onSnackOpen();
    }
    return (
        <Snackbar
            open={snackOpen}
            autoHideDuration={r && r.code == 201 ? 6000 : null}
            onClose={onSnackClose}
        >
            <div>
                {r && r.code === 201 && (
                    <Alert
                        onClose={onSnackClose}
                        severity={"success"}
                        sx={{ width: "100%" }}
                    >
                        {r && r.message ? r.message : "success !"}
                    </Alert>
                )}
                {r && r.code !== 201 && (
                    <Alert
                        onClose={onSnackClose}
                        severity={"error"}
                        sx={{ width: "100%" }}
                    >
                        {r && r.message ? r.message : "error !"}
                    </Alert>
                )}
            </div>
        </Snackbar>
    );
};

export const RuleList = (props: RuleListProps) => {
    const { name, open, onClose } = props;
    const [rows, setRows] = React.useState([]);
    const [snackOpen, setSnackOpen] = React.useState(false);
    const [rowModesModel, setRowModesModel] = React.useState<GridRowModesModel>(
        {}
    );
    const [postResource, setPostResource] = React.useState({
        read() {
            return null;
        },
    });

    const roleOrError = useFetch(["RuleList", name], () =>
        RbacClient.fetchRole(name)
    );
    const role =
        roleOrError && roleOrError.isSuccess() ? roleOrError.value : null;
    React.useEffect(() => {
        const items = toItems(role);
        setRows(items);
    }, [role]);

    const handleRowEditStart = (
        params: GridRowParams,
        event: MuiEvent<React.SyntheticEvent>
    ) => {
        event.defaultMuiPrevented = true;
    };

    const handleRowEditStop: GridEventListener<"rowEditStop"> = (
        params,
        event
    ) => {
        event.defaultMuiPrevented = true;
    };

    const handleEditClick = (id: GridRowId) => () => {
        setRowModesModel({
            ...rowModesModel,
            [id]: { mode: GridRowModes.Edit },
        });
    };

    const handleSaveClick = (id: GridRowId) => () => {
        setRowModesModel({
            ...rowModesModel,
            [id]: { mode: GridRowModes.View },
        });
    };

    const handleDeleteClick = (id: GridRowId) => () => {
        setRows(rows.filter((row) => row.id !== id));
    };

    const handleCancelClick = (id: GridRowId) => () => {
        setRowModesModel({
            ...rowModesModel,
            [id]: { mode: GridRowModes.View, ignoreModifications: true },
        });

        const editedRow = rows.find((row) => row.id === id);
        if (editedRow!.isNew) {
            setRows(rows.filter((row) => row.id !== id));
        }
    };

    const processRowUpdate = (newRow: GridRowModel) => {
        const updatedRow = { ...newRow, isNew: false, isDirty: true };
        setRows((prev) => {
            const results = rows.map((row) =>
                row.id === newRow.id ? updatedRow : row
            );
            const m = toModel(role, results);
            setPostResource(RbacClient.putRole(name, m));
            return results;
        });
        return updatedRow;
    };

    const onSnackClose = (
        event?: React.SyntheticEvent | Event,
        reason?: string
    ) => {
        if (reason === "clickaway") {
            return;
        }
        setSnackOpen(false);
        setPostResource((prev) => {
            return {
                read() {
                    return null;
                },
            };
        });
    };

    const onSnackOpen = () => {
        setSnackOpen(true);
    };

    return (
        <Modal
            open={open}
            onClose={onClose}
            aria-labelledby="modal-modal-title"
            aria-describedby="modal-modal-description"
            sx={{ zIndex: 30000 }}
        >
            <Box sx={style}>
                <Stack sx={{ flexGrow: 1 }} direction="column" spacing={2}>
                    <Typography variant="h6">{name}</Typography>
                    <Typography variant="body2">{role?.note}</Typography>
                    <Box sx={{ flexGrow: 1 }}>
                        <DataGrid
                            rows={rows}
                            columns={columns(
                                rowModesModel,
                                handleSaveClick,
                                handleCancelClick,
                                handleEditClick,
                                handleDeleteClick
                            )}
                            editMode="row"
                            rowModesModel={rowModesModel}
                            onRowModesModelChange={(newModel) =>
                                setRowModesModel(newModel)
                            }
                            onRowEditStart={handleRowEditStart}
                            onRowEditStop={handleRowEditStop}
                            processRowUpdate={processRowUpdate}
                            disableSelectionOnClick
                            experimentalFeatures={{ newEditingApi: true }}
                        />
                    </Box>
                    <Stack sx={{ width: "100%", height: 30 }} spacing={1}>
                        <PutResult
                            resource={postResource}
                            snackOpen={snackOpen}
                            onSnackOpen={onSnackOpen}
                            onSnackClose={onSnackClose}
                        />
                    </Stack>
                </Stack>
            </Box>
        </Modal>
    );
};
