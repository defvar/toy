import * as React from "react";
import { makeStyles } from "@material-ui/core/styles";
import Card from "@material-ui/core/Card";
import { INodeInnerDefaultProps, INode } from "@mrblenny/react-flow-chart";
import { ServiceCardHeader } from "./ServiceCardHeader";

const useStyles = makeStyles({
    root: {
        maxWidth: 300,
    },
});

export type NodeProps = INodeInnerDefaultProps;

const getProperties = (node: INode) => {
    if (node.properties) {
        return {
            title: node.properties.title,
            icon: node.properties.icon,
            subheader: node.properties.subheader,
        };
    } else {
        return null;
    }
};

export const Node = (props: NodeProps) => {
    const classes = useStyles();

    return (
        <Card className={classes.root}>
            <ServiceCardHeader {...getProperties(props.node)} />
        </Card>
    );
};
