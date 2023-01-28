import * as React from "react";
import { SimpleMenu, SimpleMenuProps } from "../../components/SimpleMenu";
import { LabelChips } from "../../components/LabelChips";
import PlayArrowIcon from "@mui/icons-material/PlayArrow";
import DeleteIcon from "@mui/icons-material/Delete";
import {
    DataGrid,
    GridColumns,
    GridRowId,
    GridActionsCellItem,
    GridRenderCellParams,
} from "@mui/x-data-grid";
import { Box } from "@mui/material";
import { Actions } from "../../modules/graphs";
import { useNavigate } from "react-router-dom";
import { NavigateFunction } from "react-router";
import CircularProgress from "../../components/progress/CircularProgress";
import { Result, Resource } from "../../modules/common";
import {
    GraphNodeList,
    Graph,
    ErrorMessage,
    GraphClient,
} from "../../modules/api";
import { Label } from "../../modules/api/toy-api-model";

export interface GraphListProps {
    dispatch: React.Dispatch<Actions>;
}

const menuOptions = (navigate: NavigateFunction, name): SimpleMenuProps => {
    return {
        options: [
            {
                display: "Edit",
                onClick: () => {
                    navigate(`/graphs/${name}/edit`, { replace: true });
                },
            },
            {
                display: "Log",
                onClick: () => {
                    console.log("log");
                },
            },
        ],
    };
};

const gridColumns = (props: GridProps): GridColumns<Graph> => [
    { field: "name", headerName: "name", width: 150 },
    {
        field: "labels",
        headerName: "labels",
        width: 230,
        renderCell: (params: GridRenderCellParams<Label[]>) => (
            <LabelChips labels={params.value} />
        ),
    },
    {
        field: "actions",
        type: "actions",
        getActions: (params) => [
            <GridActionsCellItem
                icon={<PlayArrowIcon />}
                onClick={props.onPlay(params.id)}
                label="Play"
            />,
            <GridActionsCellItem
                icon={<DeleteIcon />}
                onClick={props.onDelete(params.id)}
                label="Delete"
            />,
            <GridActionsCellItem
                onClick={props.onEdit(params.id)}
                label="Edit"
                showInMenu
            />,
        ],
    },
];

interface GridProps {
    resource: Resource<Result<GraphNodeList, ErrorMessage>>;
    onEdit: (id: GridRowId) => () => void;
    onPlay: (id: GridRowId) => () => void;
    onDelete: (id: GridRowId) => () => void;
}

const Grid = (prop: GridProps): JSX.Element => {
    const graphs = prop.resource.read();
    const items = graphs && graphs.isSuccess() ? graphs.value.items : [];
    const columns = gridColumns(prop);
    return (
        <DataGrid
            getRowId={(r) => r.name}
            rows={items}
            columns={columns}
            // checkboxSelection
            disableSelectionOnClick
            experimentalFeatures={{ newEditingApi: true }}
            getEstimatedRowHeight={() => 100}
            getRowHeight={() => "auto"}
        />
    );
};

export const GraphList = (props: GraphListProps) => {
    const navigate = useNavigate();
    const [graphsResource, _setGraphsResource] = React.useState(() =>
        GraphClient.fetchGraphs()
    );
    const onEdit = React.useCallback(
        (id: GridRowId) => () => {
            const name = id.toString();
            navigate(`/graphs/${name}/edit`, { replace: true });
        },
        []
    );

    const onPlay = React.useCallback(
        (id: GridRowId) => () => {
            const name = id.toString();
        },
        []
    );

    const onDelete = React.useCallback(
        (id: GridRowId) => () => {
            const name = id.toString();
        },
        []
    );

    return (
        <Box sx={{ height: 500, width: "100%", display: "flex" }}>
            <React.Suspense fallback={<CircularProgress />}>
                <Grid
                    resource={graphsResource}
                    onEdit={onEdit}
                    onPlay={onPlay}
                    onDelete={onDelete}
                />
            </React.Suspense>
        </Box>
    );
};
