import { createFileRoute } from "@tanstack/react-router";
import script from "../../../../install.sh?raw";

export const Route = createFileRoute("/install.sh")({
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
