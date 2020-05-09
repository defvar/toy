import * as React from 'react';
import { useParams } from "react-router-dom";
import { FlowChart, IChart, actions } from "@mrblenny/react-flow-chart";
import { createStyles, Theme, makeStyles } from '@material-ui/core/styles';
import Typography from '@material-ui/core/Typography';
import Grid from '@material-ui/core/Grid';
import RefreshIcon from '@material-ui/icons/Refresh';
import IconButton from '@material-ui/core/IconButton';
import { Node, Sidebar } from '../components/chart';
import ZoomInIcon from '@material-ui/icons/ZoomIn';
import ZoomOutIcon from '@material-ui/icons/ZoomOut';

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            flexGrow: 1,
        },
        heading: {
            fontSize: theme.typography.pxToRem(15),
        },
        chartCanvas: {
            overflow: 'hidden',
            maxHeight: '80vh'
        }
    }),
);

export const chartSimple: IChart = {
    offset: {
        x: 0,
        y: 0,
    },
    scale: 1,
    nodes: {
        node1: {
            id: 'node1',
            type: 'output-only',
            position: {
                x: 300,
                y: 100,
            },
            ports: {
                port1: {
                    id: 'port1',
                    type: 'output',
                    properties: {
                        value: 'yes',
                    },
                },
                port2: {
                    id: 'port2',
                    type: 'output',
                    properties: {
                        value: 'no',
                    },
                },
            },
            properties: {
                icon: 'file',
            },
        },
        node2: {
            id: 'node2',
            type: 'input-output',
            position: {
                x: 300,
                y: 300,
            },
            ports: {
                port1: {
                    id: 'port1',
                    type: 'input',
                },
                port2: {
                    id: 'port2',
                    type: 'output',
                },
            },
        },
        node3: {
            id: 'node3',
            type: 'input-output',
            position: {
                x: 100,
                y: 600,
            },
            ports: {
                port1: {
                    id: 'port1',
                    type: 'input',
                },
                port2: {
                    id: 'port2',
                    type: 'output',
                },
            },
        },
        node4: {
            id: 'node4',
            type: 'input-output',
            position: {
                x: 500,
                y: 600,
            },
            ports: {
                port1: {
                    id: 'port1',
                    type: 'input',
                },
                port2: {
                    id: 'port2',
                    type: 'output',
                },
            },
        },
    },
    links: {
        link1: {
            id: 'link1',
            from: {
                nodeId: 'node1',
                portId: 'port2',
            },
            to: {
                nodeId: 'node2',
                portId: 'port1',
            },
            properties: {
                label: 'example link label',
            },
        },
        link2: {
            id: 'link2',
            from: {
                nodeId: 'node2',
                portId: 'port2',
            },
            to: {
                nodeId: 'node3',
                portId: 'port1',
            },
            properties: {
                label: 'another example link label',
            },
        },
        link3: {
            id: 'link3',
            from: {
                nodeId: 'node2',
                portId: 'port2',
            },
            to: {
                nodeId: 'node4',
                portId: 'port1',
            },
        },
    },
    selected: {},
    hovered: {},
}

const createHandlers = (setState: React.Dispatch<React.SetStateAction<IChart>>) => Object.entries(actions)
    .reduce((r, [key, fn]) => {
        r[key] = (...args: any) => setState((prev) => {
            const res = fn(...args)(prev);
            return {
                ...res
            };
        }
        );
        return r;
    }, {}
    ) as typeof actions;

export const GraphEdit = () => {

    const { name } = useParams();
    const classes = useStyles();
    const [state, setState] = React.useState(chartSimple);
    const [handlers] = React.useState(() => createHandlers(setState));

    const handleZoomIn = () => {
        setState((prev) => ({
            ...prev,
            scale: prev.scale + 0.1,
        }));
    };

    const handleZoomOut = () => {
        setState((prev) => ({
            ...prev,
            scale: prev.scale - 0.1,
        }));
    };

    return (
        <div className={classes.root}>
            <Typography className={classes.heading}>{name}</Typography>
            <Grid container item spacing={1} direction="row" alignItems="stretch">
                <Grid item xs={9}>
                    <IconButton aria-label="refresh">
                        <RefreshIcon />
                    </IconButton>
                    <IconButton aria-label="zoom-in" onClick={handleZoomIn}>
                        <ZoomInIcon />
                    </IconButton>
                    <IconButton aria-label="zoom-out" onClick={handleZoomOut}>
                        <ZoomOutIcon />
                    </IconButton>
                    <div className={classes.chartCanvas}>
                        <FlowChart chart={state} Components={{ NodeInner: Node }} callbacks={handlers} config={{ zoom: { wheel: { disabled: true } } }} />
                    </div>
                </Grid>
                <Grid item xs={3}>
                    <IconButton aria-label="refresh">
                        <RefreshIcon />
                    </IconButton>
                    <Sidebar
                        services={{
                            "common.file.reader": { name: 'reader', namespace: 'common.file', fullName: 'common.file.reader', description: 'file read service.', inPort: 1, outPort: 1, },
                            "common.file.writer": { name: 'writer', namespace: 'common.file', fullName: 'common.file.writer', description: 'file writer service.', inPort: 1, outPort: 1, },
                            "common.map.typed": { name: 'typed', namespace: 'common.map', fullName: 'common..map.typed', description: 'aaaaaaaaaaaaaaaaa.', inPort: 1, outPort: 1, },
                            "common.map.reorder": { name: 'reorder', namespace: 'common.map', fullName: 'common.map.reorder', description: 'bbbbbbbbbbbbbbb.', inPort: 1, outPort: 1, },
                            "aiueo.ccc": { name: 'ccc', namespace: 'aiueo', fullName: 'aiueo.ccc', description: 'cccccccccccccccccccc.', inPort: 1, outPort: 1, },
                        }}
                        namespaces={{
                            "common.file": ["common.file.reader", "common.file.writer"],
                            "common.map": ["common.map.typed", "common.map.reorder"],
                            "aiueo": ["aiueo.ccc"]
                        }} />
                </Grid>
            </Grid>
        </div>
    );
}

export default GraphEdit;
