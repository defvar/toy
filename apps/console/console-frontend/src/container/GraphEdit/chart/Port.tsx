import * as React from "react";
import { IPortDefaultProps } from "@mrblenny/react-flow-chart";
import StopRoundedIcon from "@mui/icons-material/StopRounded";
import { Theme } from "@mui/material/styles";
import createStyles from '@mui/styles/createStyles';
import makeStyles from '@mui/styles/makeStyles';
import clsx from "clsx";

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            "&:hover > *": {
                fontSize: "24px",
            },
        },
    })
);

export const CustomPort = React.memo(
    ({ isLinkSelected, isLinkHovered, config }: IPortDefaultProps) => {
        const classes = useStyles();
        return (
            <div className={clsx(classes.root)}>
                {!config.readonly && (isLinkSelected || isLinkHovered) ? (
                    <StopRoundedIcon fontSize={"medium"} />
                ) : (
                    <StopRoundedIcon fontSize={"small"} />
                )}
            </div>
        );
    }
);
