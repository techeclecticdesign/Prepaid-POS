import Button, { ButtonProps } from "@mui/material/Button";

interface AppButtonProps extends ButtonProps {
  text: string;
}

// A button with custom defaults and which takes its inner text as a prop
export default function AppButton({
  variant = "contained",
  color = "primary",
  sx = { maxWidth: "14rem", width: "100%" },
  text,
  ...rest
}: AppButtonProps) {
  return (
    <Button variant={variant} color={color} sx={sx} {...rest}>
      {text}
    </Button>
  );
}
