# ROS 2 Integration Plan

## 1. Baseline

As of July 2026, ROS 2 Lyrical Luth is the current LTS release and is supported until May 2031. KX should target the current ROS 2 LTS plus Rolling in CI, while isolating distribution-specific compatibility code.

## 2. Correct integration layer

ROS 2 client libraries such as `rclcpp` and `rclpy` share common functionality in the C client layer `rcl`. KX should build `rclkx` on `rcl` and generated type support.

Do not:

- wrap all of `rclcpp` as the primary architecture;
- implement a new DDS stack;
- implement a new `rmw` vendor merely to support KX;
- make KX-to-KX communication the only test.

## 3. Package architecture

```text
rclkx/
├── core         # context, node, wait set, clock, logging
├── pubsub       # publishers, subscriptions, QoS
├── services     # client/server
├── actions      # action client/server
├── parameters
├── lifecycle
├── executor
├── tracing
└── generated    # rosidl KX type support
```

## 4. User syntax concept

```kx
use ros2::{Node, Publisher, Subscription, qos};
use geometry_msgs.msg.Twist;

fn main() -> Result<void, ros2.Error> {
    let mut node = Node.create("kx_controller")?;
    let pub = node.publisher<Twist>("cmd_vel", qos.sensor_data())?;

    node.subscribe<LaserScan>("scan", qos.sensor_data(), fn(scan) {
        let command = controller.update(scan);
        pub.publish(command)?;
    })?;

    node.spin()?;
    return ok();
}
```

The exact closure and borrowing syntax depends on the finalized memory model.

## 5. Interface generation

Extend the ROS interface-generation pipeline to produce:

- KX structures for `.msg`;
- request/response structures for `.srv`;
- goal/result/feedback structures for `.action`;
- type-support bindings;
- introspection metadata where available;
- conversion tests against C and C++ representations.

## 6. QoS

Expose ROS 2 QoS without simplifying away critical policies:

- history;
- depth;
- reliability;
- durability;
- deadline;
- lifespan;
- liveliness;
- lease duration.

Provide named profiles, but retain explicit construction.

## 7. Executors

KX should eventually provide:

- single-threaded executor;
- multithreaded executor;
- static deterministic executor;
- real-time bounded executor;
- integration with external event loops through wait sets.

Every executor documents:

- callback ordering;
- thread assignment;
- allocation behavior;
- locking behavior;
- wakeup and scheduling semantics;
- shutdown behavior.

## 8. Real-time concerns

- preallocate callback queues;
- avoid hidden message copies;
- support loaned messages when middleware permits;
- separate logging paths for real-time contexts;
- expose deadline misses and queue overrun;
- add allocation audit instrumentation;
- use bounded error handling in real-time profiles.

## 9. Coordinate frames and ROS messages

ROS messages often carry frame IDs as runtime strings. KX compile-time frames cannot automatically prove the correctness of arbitrary incoming strings.

Correct approach:

- represent trusted internal frames as types;
- validate external frame IDs at boundaries;
- use checked conversion from dynamic ROS frame IDs to typed KX frame tokens;
- never claim compile-time frame safety for unvalidated network data.

## 10. Build integration

- `ament_cmake` extension for KX compilation;
- `colcon` package discovery;
- generated package metadata;
- environment hooks for KX libraries;
- version compatibility with selected ROS distributions.

## 11. Required integration tests

- KX publisher -> C++ subscriber;
- Python publisher -> KX subscriber;
- KX service server -> C++ and Python clients;
- KX action client -> C++ action server;
- custom interface round trip;
- QoS compatible/incompatible cases;
- lifecycle transitions;
- executor shutdown and cancellation;
- high-rate message latency and loss;
- cross-process and cross-host tests.
