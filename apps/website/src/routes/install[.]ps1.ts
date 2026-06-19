import { createFileRoute } from "@tanstack/react-router";
import script from "../../../../install.ps1?raw";

export const Route = createFileRoute("/install.ps1")({
  server: {
    handlers: {
      GET: () => {
        return new Response(script, {
          headers: {
            "Content-Type": "text/plain; charset=utf-8",
          },
        });
      },
    },
  },
});
