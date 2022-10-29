import * as React from "react";
import { Box, Modal } from "@mui/material";
import {
    DataGrid,
    GridColumns,
    GridRowParams,
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
import { Role } from "../../modules/api";
import { Resource } from "../../modules/common";

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

const style = {
    position: "relative" as "relative",
    top: "50%",
    left: "50%",
    transform: "translate(-50%, -50%)",
    height: "400px",
    width: "100%",
    maxWidth: "1000px",
    bgcolor: "background.paper",
    border: "2px solid #000",
    boxShadow: 24,
    p: 4,
};

export interface RuleListProps {
    name: string;
    resource: Resource<Role>;
    open: boolean;
    onClose: () => void;
}

export const RuleList = (props: RuleListProps) => {
    const { name, resource, open, onClose } = props;
    const [rows, setRows] = React.useState([]);
    const [rowModesModel, setRowModesModel] = React.useState<GridRowModesModel>(
        {}
    );

    const role = resource ? resource.read() : null;
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
        const updatedRow = { ...newRow, isNew: false };
        setRows(rows.map((row) => (row.id === newRow.id ? updatedRow : row)));
        return updatedRow;
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
        </Modal>
    );
};
