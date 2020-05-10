import * as React from "react";
import { makeStyles, Theme, createStyles } from "@material-ui/core/styles";
import Card from "@material-ui/core/Card";
import { INodeInnerDefaultProps, INode } from "@mrblenny/react-flow-chart";
import { ServiceCardHeader } from "./ServiceCardHeader";

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            maxWidth: 300,
        },
    })
);

export type NodeProps = INodeInnerDefaultProps;

const getProperties = (node: INode) => {
    if (node.properties) {
        return {
            fullName: node.properties.fullName,
            name: node.properties.name,
            icon: node.properties.icon,
            namespace: node.properties.namespace,
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
