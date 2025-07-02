import { Button, ButtonProps } from "@mui/material";

interface AppButtonProps extends ButtonProps {
  text: string;
}

// A button with custom defaults and which takes its inner text as a prop
export default function AppButton({
  variant = "outlined",
  color = "primary",
  sx = { width: 250 },
  text,
  ...rest
}: AppButtonProps) {
  return (
    <Button variant={variant} color={color} sx={sx} {...rest}>
      {text}
    </Button>
  );
}
