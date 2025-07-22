export const formatDate = (dateString: string | null | undefined) => {
  if (!dateString) {
    return "-";
  }
  return new Date(dateString).toLocaleDateString();
};

export const formatCurrency = (cents: number) => {
  return `$${(cents / 100).toFixed(2)}`;
};
