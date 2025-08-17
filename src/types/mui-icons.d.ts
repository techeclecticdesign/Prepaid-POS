declare module "@mui/icons-material/*" {
  import * as React from "react";
  const Icon: React.ComponentType<
    React.SVGProps<SVGSVGElement> & {
      fontSize?: "inherit" | "small" | "medium" | "large";
    }
  >;
  export default Icon;
}
