import * as React from "react";
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
import Box from "@mui/material/Box";
import Switch from "@mui/material/Switch";
import { Actions } from "../../modules/graphs";
import { useNavigate } from "react-router-dom";
import CircularProgress from "../../components/progress/CircularProgress";
import { Result, Resource } from "../../modules/common";
import {
    GraphNodeList,
    Graph,
    ErrorMessage,
    GraphClient,
    Label,
} from "../../modules/api";

export interface GraphListProps {
    dispatch: React.Dispatch<Actions>;
}

const gridColumns = (props: GridProps): GridColumns<Graph> => [
    {
        field: "disabled",
        width: 100,
        renderCell: (params: GridRenderCellParams<boolean>) => (
            <Switch
                checked={!params.value}
                name={params.row.name}
                onChange={props.onChangeDisabled}
            />
        ),
    },
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
    onChangeDisabled: (event: React.ChangeEvent<HTMLInputElement>) => void;
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

    const onChangeDisabled = React.useCallback(
        (event: React.ChangeEvent<HTMLInputElement>) => {
            const name = event.target.name;
            // update disabled

            // refresh
        },
        []
    );

    return (
        <Box sx={{ height: 500, width: "100%", display: "flex" }}>
            <React.Suspense fallback={<CircularProgress />}>
                <Grid
                    resource={graphsResource}
                    onChangeDisabled={onChangeDisabled}
                    onEdit={onEdit}
                    onPlay={onPlay}
                    onDelete={onDelete}
                />
            </React.Suspense>
        </Box>
    );
};
