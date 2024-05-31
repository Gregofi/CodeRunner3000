import Alert from "@mui/material/Alert";
import Button from "@mui/material/Button";
import Snackbar from "@mui/material/Snackbar";
import React from "react";

export default function ShareButton({ onClick }: Readonly<{ onClick(): boolean }>) {
    const [open, setOpen] = React.useState<"success" | "error" | "closed">("closed");

    const handleClick = () => {
        if (open === "closed") {
            if (onClick()) {
                navigator.clipboard.writeText("http://localhost:3000/abcd");
                setOpen("success");
            } else {
                // error message in snackbar
                setOpen("error");
            }
        }
    }

    const handleClose = () => {
        setOpen("closed");
    }

    const alertSeverity = open !== "closed" ? open : "success";

    return (
        <>
            <Button variant="contained" sx={{borderRadius: 0}} onClick={handleClick}>Share</Button>
            <Snackbar open={open !== "closed"} autoHideDuration={3000} onClose={handleClose}>
                <Alert severity={alertSeverity} sx={{width: "100%"}} onClose={handleClose}>
                    {alertSeverity === "success" && "Copied to clipboard"}
                    {alertSeverity === "error" && "Failed to create link"}
                </Alert>
            </Snackbar>
        </>
    )
}
