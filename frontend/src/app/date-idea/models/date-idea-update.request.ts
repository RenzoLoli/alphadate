export interface DateIdeaUpdateRequest {
  idea?: string;
  description?: string;
}

export const checkUnnecessaryDateIdeaUpdateRequest = (
  dateIdeaUpdateRequest: DateIdeaUpdateRequest,
) => {
  if (dateIdeaUpdateRequest.idea === undefined) {
    return false;
  }
  if (dateIdeaUpdateRequest.description === undefined) {
    return false;
  }
  return true;
};
