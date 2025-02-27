// Copyright @ 2023 - 2024, R3E Network
// All Rights Reserved

// Sandbox permissions API
export async function requestPermission(operation, resource = null) {
  const request = {
    operation,
    resource,
  };
  
  const response = await Deno.core.ops.op_request_permission(request);
  
  if (!response.granted) {
    throw new Error(response.message || `Permission denied: ${operation}`);
  }
  
  return true;
}

// Export sandbox API
export const sandbox = {
  requestPermission,
};
