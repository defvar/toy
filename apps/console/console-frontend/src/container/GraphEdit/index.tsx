import * as React from "react";
import { useParams } from "react-router-dom";
import { Theme, useTheme, styled } from "@mui/material/styles";
import createStyles from "@mui/styles/createStyles";
import makeStyles from "@mui/styles/makeStyles";
import Typography from "@mui/material/Typography";
import RefreshIcon from "@mui/icons-material/Refresh";
import IconButton from "@mui/material/IconButton";
import Grid from "@mui/material/Grid";
import Box from "@mui/material/Box";
import Drawer from "@mui/material/Drawer";
import Toolbar from "@mui/material/Toolbar";
import Divider from "@mui/material/Divider";
import { Sidebar, Chart } from "./chart";
import CircularProgress from "@mui/material/CircularProgress";
import { Resource } from "../../modules/common";
import {
    GraphEditState,
    Actions,
    reducer,
    initialState,
    ServiceState,
} from "../../modules/graphEdit";
import {
    ServiceResponse,
    fetchServices,
    fetchGraph,
    GraphResponse,
} from "../../modules/api/toy-api";
import { NodeEditor } from "./NodeEditor";
import { ChartData } from "../../modules/graphEdit/types";
import DrawerHeader from "../../components/DrawerHeader";
import Paper from "@mui/material/Paper";

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        loader: {
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
        },
        progress: {},
    })
);

interface GraphEditSuspenseProps {
    state: GraphEditState;
    dispatch: React.Dispatch<Actions>;
    serviceResource: Resource<ServiceResponse>;
    graphResource: Resource<GraphResponse>;
}

const _testServices = {
    "a.b.c": {
        fullName: "a.b.c",
        name: "c",
        namespace: "a.b",
        description: "aaaaaa",
        inPort: 1,
        outPort: 1,
        configSchema: null,
        portType: "Flow",
    },
} as { [fullName: string]: ServiceState };

const _testNamespaces = {
    "a.b": ["a.b.c"],
};

const _testChartData = {
    elements: [
        {
            id: "tick",
            type: "input",
            position: {
                x: 250,
                y: 0,
            },
            data: {
                name: "tick",
                label: "tick",
                fullName: "plugin.common.timer.tick",
                dirty: false,
                portType: "Source",
            },
        },
        {
            id: "broadcast",
            type: "default",
            position: {
                x: 250,
                y: 150,
            },
            data: {
                name: "broadcast",
                label: "broadcast",
                fullName: "plugin.common.fanout.broadcast",
                dirty: false,
                portType: "Flow",
            },
        },
        {
            id: "last",
            type: "output",
            position: {
                x: 250,
                y: 250,
            },
            data: {
                name: "last",
                label: "last",
                fullName: "plugin.common.collect.last",
                dirty: false,
                portType: "Sink",
            },
        },
        {
            id: "count",
            type: "output",
            position: {
                x: 500,
                y: 250,
            },
            data: {
                name: "count",
                label: "count",
                fullName: "plugin.common.collect.count",
                dirty: false,
                portType: "Sink",
            },
        },
        {
            id: "out",
            type: "output",
            position: {
                x: 750,
                y: 250,
            },
            data: {
                name: "stdout",
                label: "stdout",
                fullName: "plugin.common.stdio.stdout",
                dirty: false,
                portType: "Sink",
            },
        },
        {
            id: "link-tick-broadcast",
            source: "tick",
            target: "broadcast",
        },
        {
            id: "link-broadcast-last",
            source: "broadcast",
            target: "last",
        },
        {
            id: "link-broadcast-count",
            source: "broadcast",
            target: "count",
        },
        {
            id: "link-broadcast-out",
            source: "broadcast",
            target: "out",
        },
    ],
} as ChartData;

const SidebarSuspense = (props: GraphEditSuspenseProps) => {
    const r = props.serviceResource.read();
    React.useEffect(() => {
        props.dispatch({
            type: "GetServices",
            payload: r,
        });
    }, [r]);
    return (
        <Sidebar
            services={props.state.services}
            namespaces={props.state.namespaces}
        />
    );
};

const ChartSuspense = (props: GraphEditSuspenseProps) => {
    const graph = props.graphResource.read();
    React.useEffect(() => {
        props.dispatch({
            type: "GetGraph",
            payload: graph,
        });
    }, [graph]);

    return (
        <Chart
            data={_testChartData /*props.state.chart*/}
            dispatch={props.dispatch}
        />
    );
};

export const GraphEdit = () => {
    const { name } = useParams<{ name: string }>();
    const classes = useStyles();
    const theme = useTheme();

    const [serviceResource, setServiceResource] = React.useState(() =>
        fetchServices()
    );
    const [graphResource, setGraphResource] = React.useState(() =>
        fetchGraph(name)
    );
    const [state, dispatch] = React.useReducer(reducer, initialState);
    const [tabNumber, setTabNumber] = React.useState(0);

    const onChartRefleshClick = React.useCallback(() => {
        setGraphResource(() => fetchGraph(name));
    }, []);

    const onSidebarRefleshClick = React.useCallback(() => {
        setServiceResource(() => fetchServices());
    }, []);

    const [size, setSize] = React.useState(() => {
        return { width: theme.breakpoints.values.md };
    });

    return (
        <Box>
            <Grid container spacing={2}>
                <Grid item xs={10}>
                    <Typography
                        sx={{ marginBottom: 2 }}
                        variant="h6"
                        component="div"
                    >
                        {name}
                    </Typography>
                    <Paper elevation={2}>
                        <IconButton
                            aria-label="refresh"
                            onClick={onChartRefleshClick}
                            size="large"
                        >
                            <RefreshIcon />
                        </IconButton>
                        <React.Suspense
                            fallback={
                                <div className={classes.loader}>
                                    <CircularProgress
                                        size={68}
                                        className={classes.progress}
                                    />
                                </div>
                            }
                        >
                            <ChartSuspense
                                state={state}
                                dispatch={dispatch}
                                serviceResource={serviceResource}
                                graphResource={graphResource}
                            />
                        </React.Suspense>
                    </Paper>
                </Grid>
                <Grid item xs={2}>
                    <Drawer variant="permanent" anchor={"right"} open={true}>
                        <Toolbar />
                        <Divider />
                        <DrawerHeader>
                            <IconButton
                                aria-label="refresh"
                                onClick={onSidebarRefleshClick}
                                size="large"
                            >
                                <RefreshIcon />
                            </IconButton>
                        </DrawerHeader>
                        <React.Suspense
                            fallback={
                                <div className={classes.loader}>
                                    <CircularProgress
                                        size={68}
                                        className={classes.progress}
                                    />
                                </div>
                            }
                        >
                            <SidebarSuspense
                                state={state}
                                dispatch={dispatch}
                                serviceResource={serviceResource}
                                graphResource={graphResource}
                            />
                        </React.Suspense>
                    </Drawer>
                </Grid>
            </Grid>
            <NodeEditor
                state={state}
                dispatch={dispatch}
                open={!!state.edit.id}
            />
        </Box>
    );
};

export default GraphEdit;
