const FLT_MAX: f32 = 3.40282346638528859812e+38;
const EPSILON: f32 = 1e-3;

const MAX_PATH_LENGTH: u32 = 6u;

struct Uniforms {
  width: u32,
  height: u32,
  frame_count: u32,
}
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct Rng {
  state: u32,
};
var<private> rng: Rng;

fn init_rng(pixel: vec2u) {
  let seed = (pixel.x + pixel.y * uniforms.width) ^ jenkins_hash(uniforms.frame_count);
  rng.state = jenkins_hash(seed);
}

fn jenkins_hash(i: u32) -> u32 {
  var x = i;
  x += x << 10u;
  x ^= x >> 6u;
  x += x << 3u;
  x ^= x >> 11u;
  x += x << 15u;
  return x;
}

fn xorshift32() -> u32 {
  var x = rng.state;
  x ^= x << 13;
  x ^= x >> 17;
  x ^= x << 5;
  rng.state = x;
  return x;
}

fn rand_f32() -> f32 {
  return bitcast<f32>(0x3f800000u | (xorshift32() >> 9u)) - 1.;
}

struct Intersection {
  normal: vec3f,
  t: f32,
  c: f32,
}

fn no_intersection() -> Intersection {
  return Intersection(vec3(0.), -1.,-1.);
}

fn is_intersection_valid(hit: Intersection) -> bool {
  return hit.t > 0.;
}

struct Sphere {
  center: vec3f,
  radius: f32,
}

fn intersect_sphere(ray: Ray, sphere: Sphere) -> Intersection {
  let v = ray.origin - sphere.center;
  let a = dot(ray.direction, ray.direction);
  let b = dot(v, ray.direction);
  let c = dot(v, v) - sphere.radius * sphere.radius;

  let d = b * b - a * c;
  if d < 0. {
    return no_intersection();
  }

  let sqrt_d = sqrt(d);
  let recip_a = 1. / a;
  let mb = -b;
  let t1 = (mb - sqrt_d) * recip_a;
  let t2 = (mb + sqrt_d) * recip_a;
  let t = select(t2, t1, t1 > EPSILON);
  if t <= EPSILON {
    return no_intersection();
  }

  let p = point_on_ray(ray, t);
  let N = (p - sphere.center) / sphere.radius;
  return Intersection(N, t,-1.0);
}

fn intersect_scene(ray: Ray) -> Intersection {
  var closest_hit = Intersection(vec3(0.), FLT_MAX,-1.0);
  for (var i = 0u; i < OBJECT_COUNT; i += 1u) {
    let sphere = scene[i];
    var hit = intersect_sphere(ray, sphere);
    if i==1u {
      hit.c=1.0;
    }
    if hit.t > 0. && hit.t < closest_hit.t {
      closest_hit = hit;
    }
  }
  if closest_hit.t < FLT_MAX {
    return closest_hit;
  }
  return no_intersection();
}

struct Scatter {
  attenuation: vec3f,
  ray: Ray,
}

fn refract(I: vec3f, N: vec3f, eta: f32) -> vec3f {
  let cos_i = dot(-I, N);
  let sin2_t = eta * eta * (1.0 - cos_i * cos_i);
  if sin2_t > 1.0 {
    return vec3(0.0, 0.0, 0.0);
  }
  let cos_t = sqrt(1.0 - sin2_t);
  return eta * I + (eta * cos_i - cos_t) * N;
}
fn reflect(I: vec3f, N: vec3f) -> vec3f {
  return I - 2.0 * dot(I, N) * N;
}
fn scatter(input_ray: Ray, hit: Intersection) -> Scatter {
  var output_ray: Ray;
  let attenuation = vec3(0.5);
  var eta = 0.5;
    if dot(input_ray.direction, hit.normal) > 0. {
    eta = 1. / eta;
    }
  if (rand_f32() < 0.8) {
    let reflected = reflect(input_ray.direction, hit.normal);
    output_ray = Ray(point_on_ray(input_ray, hit.t), hit.normal);
  } else {
    let refracted = refract(input_ray.direction, hit.normal, eta);
    output_ray = Ray(point_on_ray(input_ray, hit.t), refracted);
  }
  return Scatter(attenuation, output_ray);
}

struct Ray {
  origin: vec3f,
  direction: vec3f,
}

fn point_on_ray(ray: Ray, t: f32) -> vec3<f32> {
  return ray.origin + t * ray.direction;
}

fn sky_color(ray: Ray) -> vec3f {
  let t = 0.5 * (normalize(ray.direction).y + 1.);
  return (1. - t) * vec3(1.) + t * vec3(0.3, 0.5, 1.);
}

const OBJECT_COUNT: u32 = 2;
alias Scene = array<Sphere, OBJECT_COUNT>;
var<private> scene: Scene = Scene(
  Sphere(vec3( 0.0,0.5,-0.5 ),0.5),
  Sphere(vec3(0.0,0.5, 100.5),100.),
);

@group(0) @binding(1) var radiance_samples_old: texture_2d<f32>;
@group(0) @binding(2) var radiance_samples_new: texture_storage_2d<rgba32float, write>;

alias TriangleVertices = array<vec2f, 6>;
var<private> vertices: TriangleVertices = TriangleVertices(
  vec2f(-1.0,  1.0),
  vec2f(-1.0, -1.0),
  vec2f( 1.0,  1.0),
  vec2f( 1.0,  1.0),
  vec2f(-1.0, -1.0),
  vec2f( 1.0, -1.0),
);

@vertex fn display_vs(@builtin(vertex_index) vid: u32) -> @builtin(position) vec4f {
  return vec4f(vertices[vid], 0.0, 1.0);
}

@fragment fn display_fs(@builtin(position) pos: vec4f) -> @location(0) vec4f {
  init_rng(vec2u(pos.xy));

  let origin = vec3(0.);
  let focus_distance = 1.;
  let aspect_ratio = f32(uniforms.width) / f32(uniforms.height);

  let offset = vec2(rand_f32() - 0.5, rand_f32() - 0.5);
  var uv = (pos.xy + offset) / vec2f(f32(uniforms.width - 1u), f32(uniforms.height - 1u));

  uv = (2. * uv - vec2(1.)) * vec2(aspect_ratio, -1.);

  let direction = vec3(uv, -focus_distance);
  var ray = Ray(origin, direction);
  var throughput = vec3f(1.);
  var radiance_sample = vec3(0.);

  var path_length = 0u;
  while path_length < MAX_PATH_LENGTH {
    let hit = intersect_scene(ray);
    if hit.c>0{
    radiance_sample += throughput * vec3(0., 7., 0.);
    }
    else
    {if is_intersection_valid(hit) {
      let scattered = scatter(ray, hit);
      throughput *= scattered.attenuation;
      ray = scattered.ray;
    } else {
      radiance_sample += throughput * sky_color(ray);
      break;
    }}
    path_length += 1u;
  }

  var old_sum: vec3f;
  if uniforms.frame_count > 1 {
    old_sum = textureLoad(radiance_samples_old, vec2u(pos.xy), 0).xyz;
  } else {
    old_sum = vec3(0.);
  }

  let new_sum = radiance_sample + old_sum;
  textureStore(radiance_samples_new, vec2u(pos.xy), vec4(new_sum, 0.));

  return vec4(new_sum / f32(uniforms.frame_count), 1.);
}
