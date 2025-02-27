/**
 * Mock Metrics Data for Neo N3 FaaS Platform GraphQL API
 * 
 * This file provides mock data and operations for function metrics in the Neo N3 FaaS platform.
 */

// Mock metrics data
const metrics = {
  'function-1': {
    invocations: {
      total: 1250,
      success: 1200,
      failed: 50,
      avgDuration: 950
    },
    resources: {
      memory: {
        avg: 85.5,
        peak: 120.3
      },
      cpu: {
        avg: 15.2,
        peak: 45.8
      }
    }
  },
  'function-2': {
    invocations: {
      total: 980,
      success: 950,
      failed: 30,
      avgDuration: 850
    },
    resources: {
      memory: {
        avg: 75.2,
        peak: 110.5
      },
      cpu: {
        avg: 12.8,
        peak: 38.6
      }
    }
  },
  'function-3': {
    invocations: {
      total: 2500,
      success: 2450,
      failed: 50,
      avgDuration: 750
    },
    resources: {
      memory: {
        avg: 65.8,
        peak: 95.2
      },
      cpu: {
        avg: 10.5,
        peak: 35.2
      }
    }
  },
  'function-4': {
    invocations: {
      total: 1800,
      success: 1750,
      failed: 50,
      avgDuration: 650
    },
    resources: {
      memory: {
        avg: 55.3,
        peak: 85.7
      },
      cpu: {
        avg: 8.9,
        peak: 30.1
      }
    }
  },
  'function-5': {
    invocations: {
      total: 950,
      success: 900,
      failed: 50,
      avgDuration: 1250
    },
    resources: {
      memory: {
        avg: 125.6,
        peak: 180.3
      },
      cpu: {
        avg: 25.8,
        peak: 65.2
      }
    }
  },
  'function-6': {
    invocations: {
      total: 750,
      success: 650,
      failed: 100,
      avgDuration: 1150
    },
    resources: {
      memory: {
        avg: 115.2,
        peak: 170.5
      },
      cpu: {
        avg: 22.5,
        peak: 60.8
      }
    }
  }
};

// Get metrics by function ID
function getMetricsByFunctionId(functionId) {
  const functionMetrics = metrics[functionId];
  
  if (!functionMetrics) {
    // Return default metrics if not found
    return {
      invocations: {
        total: 0,
        success: 0,
        failed: 0,
        avgDuration: 0
      },
      resources: {
        memory: {
          avg: 0,
          peak: 0
        },
        cpu: {
          avg: 0,
          peak: 0
        }
      }
    };
  }
  
  return functionMetrics;
}

// Update metrics for a function
function updateMetrics(functionId, execution) {
  // Get current metrics
  let functionMetrics = metrics[functionId];
  
  // Create default metrics if not found
  if (!functionMetrics) {
    functionMetrics = {
      invocations: {
        total: 0,
        success: 0,
        failed: 0,
        avgDuration: 0
      },
      resources: {
        memory: {
          avg: 0,
          peak: 0
        },
        cpu: {
          avg: 0,
          peak: 0
        }
      }
    };
  }
  
  // Update invocation metrics
  functionMetrics.invocations.total += 1;
  
  if (execution.status === 'success') {
    functionMetrics.invocations.success += 1;
  } else {
    functionMetrics.invocations.failed += 1;
  }
  
  // Update average duration
  const totalDuration = functionMetrics.invocations.avgDuration * (functionMetrics.invocations.total - 1) + execution.duration;
  functionMetrics.invocations.avgDuration = totalDuration / functionMetrics.invocations.total;
  
  // Simulate resource usage
  const memoryUsage = Math.random() * 100 + 50; // 50-150 MB
  const cpuUsage = Math.random() * 30 + 5; // 5-35%
  
  // Update resource metrics
  functionMetrics.resources.memory.avg = (functionMetrics.resources.memory.avg * (functionMetrics.invocations.total - 1) + memoryUsage) / functionMetrics.invocations.total;
  functionMetrics.resources.memory.peak = Math.max(functionMetrics.resources.memory.peak, memoryUsage);
  
  functionMetrics.resources.cpu.avg = (functionMetrics.resources.cpu.avg * (functionMetrics.invocations.total - 1) + cpuUsage) / functionMetrics.invocations.total;
  functionMetrics.resources.cpu.peak = Math.max(functionMetrics.resources.cpu.peak, cpuUsage);
  
  // Update metrics
  metrics[functionId] = functionMetrics;
  
  return functionMetrics;
}

module.exports = {
  getMetricsByFunctionId,
  updateMetrics
};
