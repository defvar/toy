import * as React from "react";
import { useParams } from "react-router-dom";
import { createStyles, Theme, makeStyles } from "@material-ui/core/styles";
import Typography from "@material-ui/core/Typography";
import Grid from "@material-ui/core/Grid";
import RefreshIcon from "@material-ui/icons/Refresh";
import IconButton from "@material-ui/core/IconButton";
import {
    Sidebar,
    Chart,
    ChartData,
    initialChartData,
    toChartData,
} from "./chart";
import ZoomInIcon from "@material-ui/icons/ZoomIn";
import ZoomOutIcon from "@material-ui/icons/ZoomOut";
import CircularProgress from "@material-ui/core/CircularProgress";
import { toResource, Resource } from "../../modules/common";
import {
    GraphEditState,
    ServiceState,
    GraphState,
    Actions,
    reducer,
    initialState,
} from "../../modules/graphEdit";

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            flexGrow: 1,
        },
        heading: {
            fontSize: theme.typography.pxToRem(15),
        },
        chartCanvas: {
            overflow: "hidden",
            maxHeight: "80vh",
        },
        loader: {
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
        },
        progress: {},
    })
);

const serviceData = {
    services: {
        "common.file.reader": {
            name: "reader",
            namespace: "common.file",
            fullName: "common.file.reader",
            description: "file read service.",
            inPort: 1,
            outPort: 1,
        },
        "common.file.writer": {
            name: "writer",
            namespace: "common.file",
            fullName: "common.file.writer",
            description: "file writer service.",
            inPort: 1,
            outPort: 1,
        },
        "common.map.typed": {
            name: "typed",
            namespace: "common.map",
            fullName: "common..map.typed",
            description: "aaaaaaaaaaaaaaaaa.",
            inPort: 1,
            outPort: 1,
        },
        "common.map.reorder": {
            name: "reorder",
            namespace: "common.map",
            fullName: "common.map.reorder",
            description: "bbbbbbbbbbbbbbb.",
            inPort: 1,
            outPort: 1,
        },
        "aiueo.ccc": {
            name: "ccc",
            namespace: "aiueo",
            fullName: "aiueo.ccc",
            description: "cccccccccccccccccccc.",
            inPort: 1,
            outPort: 1,
        },
    },
    namespaces: {
        "common.file": ["common.file.reader", "common.file.writer"],
        "common.map": ["common.map.typed", "common.map.reorder"],
        aiueo: ["aiueo.ccc"],
    },
};

const graphData = {
    nodes: {
        reader1: {
            uri: "reader1",
            fullName: "common.file.reader",
            name: "reader",
            namespace: "common.file",
            description: "",
            inPort: 1,
            outPort: 1,
            position: {
                x: 300,
                y: 100,
            },
        },
        writer1: {
            uri: "writer1",
            fullName: "common.file.writer",
            name: "writer",
            namespace: "common.file",
            description: "",
            inPort: 1,
            outPort: 1,
            position: {
                x: 300,
                y: 300,
            },
        },
    },
    wires: {
        reader1: ["writer1"],
    },
};

const fetchServices = () => {
    const promise = new Promise<{
        services: { [fullName: string]: ServiceState };
        namespaces: { [namespace: string]: string[] };
    }>((resolve) => {
        setTimeout(() => {
            resolve(serviceData);
        }, 2000);
    });
    return toResource(async () => await promise);
};

const fetchGraph = (name: string) => {
    const promise = new Promise<GraphState>((resolve) => {
        setTimeout(() => {
            resolve(graphData);
        }, 3000);
    });
    return toResource(async () => await promise);
};

interface GraphEditSuspenseProps {
    state: GraphEditState;
    chartState: ChartData;
    setChartState: React.Dispatch<React.SetStateAction<ChartData>>;
    dispatch: React.Dispatch<Actions>;
    serviceResource: Resource<{
        services: { [fullName: string]: ServiceState };
        namespaces: { [namespace: string]: string[] };
    }>;
    graphResource: Resource<GraphState>;
}

const SidebarSuspense = (props: GraphEditSuspenseProps) => {
    const { services, namespaces } = props.serviceResource.read();
    React.useEffect(() => {
        props.dispatch({
            type: "GetServices",
            payload: { services, namespaces },
        });
    }, [services, namespaces]);
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
            payload: { graph },
        });
        const d = toChartData(graph);
        props.setChartState(d);
    }, [graph]);

    return <Chart data={props.chartState} dispatch={props.setChartState} />;
};

export const GraphEdit = () => {
    const { name } = useParams();
    const classes = useStyles();
    const [serviceResource, setServiceResource] = React.useState(() =>
        fetchServices()
    );
    const [graphResource, setGraphResource] = React.useState(() =>
        fetchGraph(name)
    );
    const [state, dispatch] = React.useReducer(reducer, initialState);

    const [chartState, setChartState] = React.useState(initialChartData);

    const onChartRefleshClick = React.useCallback(() => {
        setGraphResource(() => fetchGraph(name));
    }, []);

    const onSidebarRefleshClick = React.useCallback(() => {
        setServiceResource(() => fetchServices());
    }, []);

    const handleZoomIn = () => {
        setChartState((prev) => ({
            ...prev,
            scale: prev.scale + 0.1,
        }));
    };

    const handleZoomOut = () => {
        setChartState((prev) => ({
            ...prev,
            scale: prev.scale - 0.1,
        }));
    };

    return (
        <div className={classes.root}>
            <Typography className={classes.heading}>{name}</Typography>
            <Grid
                container
                item
                spacing={1}
                direction="row"
                alignItems="stretch"
            >
                <Grid item xs={9}>
                    <IconButton
                        aria-label="refresh"
                        onClick={onChartRefleshClick}
                    >
                        <RefreshIcon />
                    </IconButton>
                    <IconButton aria-label="zoom-in" onClick={handleZoomIn}>
                        <ZoomInIcon />
                    </IconButton>
                    <IconButton aria-label="zoom-out" onClick={handleZoomOut}>
                        <ZoomOutIcon />
                    </IconButton>
                    <div className={classes.chartCanvas}>
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
                                chartState={chartState}
                                setChartState={setChartState}
                                dispatch={dispatch}
                                serviceResource={serviceResource}
                                graphResource={graphResource}
                            />
                        </React.Suspense>
                    </div>
                </Grid>
                <Grid item xs={3}>
                    <IconButton
                        aria-label="refresh"
                        onClick={onSidebarRefleshClick}
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
                        <SidebarSuspense
                            state={state}
                            chartState={chartState}
                            setChartState={setChartState}
                            dispatch={dispatch}
                            serviceResource={serviceResource}
                            graphResource={graphResource}
                        />
                    </React.Suspense>
                </Grid>
            </Grid>
        </div>
    );
};

export default GraphEdit;
