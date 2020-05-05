import * as React from 'react';
import { CodeEditor } from "../components/CodeEditor";
import { useParams } from "react-router-dom";
import { FlowChartWithState, IChart, INode, REACT_FLOW_CHART } from "@mrblenny/react-flow-chart";
import { createStyles, Theme, makeStyles } from '@material-ui/core/styles';
import Typography from '@material-ui/core/Typography';
import Grid from '@material-ui/core/Grid';

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
            maxHeight: '90vh'
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

export const GraphEdit = () => {
    const { name } = useParams();
    const classes = useStyles();
    return (
        <div className={classes.root}>
            <Grid container item spacing={1} direction="row" alignItems="stretch">
                <Grid item xs={10}>
                    <Typography className={classes.heading}>{name}</Typography>
                    <div className={classes.chartCanvas}>
                        <FlowChartWithState initialValue={chartSimple} />
                    </div>
                </Grid>
                <Grid item xs={2}>
                    <div style={{ backgroundColor: 'red', fontSize: '14px' }}>
                        <SidebarItem
                            type="top/bottom"
                            ports={{
                                port1: {
                                    id: 'port1',
                                    type: 'top',
                                    properties: {
                                        custom: 'property',
                                    },
                                },
                                port2: {
                                    id: 'port1',
                                    type: 'bottom',
                                    properties: {
                                        custom: 'property',
                                    },
                                },
                            }}
                            properties={{
                                custom: 'property',
                            }}
                        />
                    </div>
                </Grid>
            </Grid>
        </div>
    );
};

export interface ISidebarItemProps {
    type: string,
    ports: INode['ports'],
    properties?: any,
}

export const SidebarItem = ({ type, ports, properties }: ISidebarItemProps) => {
    return (
        <div style={{ backgroundColor: 'white', fontSize: '14px' }}
            draggable={true}
            onDragStart={(event) => {
                event.dataTransfer.setData(REACT_FLOW_CHART, JSON.stringify({ type, ports, properties }))
            }}
        >
            {type}
        </div>
    )
}

export default GraphEdit;
