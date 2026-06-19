import { createFileRoute } from "@tanstack/react-router";
import script from "../../../../uninstall.sh?raw";

export const Route = createFileRoute("/uninstall.sh")({
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
