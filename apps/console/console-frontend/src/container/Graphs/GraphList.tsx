import * as React from "react";
import { SimpleMenu, SimpleMenuProps } from "../../components/SimpleMenu";
import { LabelChips } from "../../components/LabelChips";
import {
    DataGrid,
    GridColumns,
    GridRowId,
    GridActionsCellItem,
} from "@mui/x-data-grid";
import { Box, Tab, Tabs, Stack, Link } from "@mui/material";
import { GraphListItemState, Actions } from "../../modules/graphs";
import { useNavigate } from "react-router-dom";
import { NavigateFunction } from "react-router";
import CircularProgress from "../../components/progress/CircularProgress";
import { Result, Resource } from "../../modules/common";
import {
    GraphNodeList,
    Graph,
    ErrorMessage,
    GraphClient,
    GraphNode,
} from "../../modules/api";

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
        field: "actions",
        type: "actions",
        getActions: (params) => [
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
            checkboxSelection
            disableSelectionOnClick
            experimentalFeatures={{ newEditingApi: true }}
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

    return (
        <Box sx={{ height: 500, width: "100%", display: "flex" }}>
            <React.Suspense fallback={<CircularProgress />}>
                <Grid resource={graphsResource} onEdit={onEdit} />
            </React.Suspense>
        </Box>
    );
};
