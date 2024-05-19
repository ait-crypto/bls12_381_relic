#include "wrapper.h"

#include <stdbool.h>

static bool core_init_run = false;
static bn_t order;

__attribute__((constructor)) static void init_relic(void) {
  if (!core_get()) {
    core_init();
    core_init_run = true;
  }

  ep_param_set_any_pairf();

  bn_null(order);
  bn_new(order);
  ep_curve_get_ord(order);
}

__attribute__((destructor)) static void clean_relic(void) {
  if (core_init_run) {
    bn_free(order);

    core_init_run = false;
    core_clean();
  }
}

int wrapper_bn_init(wrapper_bn_t* bn) {
  RLC_TRY {
    bn_null(bn->value);
    bn_new(bn->value);
  }
  RLC_CATCH_ANY {
    bn_free(bn->value);
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_free(wrapper_bn_t* bn) {
  RLC_TRY {
    (void*)bn;
    bn_free(bn->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }
  return RLC_OK;
}

int wrapper_bn_copy(wrapper_bn_t* dst, const wrapper_bn_t* src) {
  RLC_TRY {
    bn_copy(dst->value, src->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }
  return RLC_OK;
}

int wrapper_bn_zero(wrapper_bn_t* bn) {
  RLC_TRY {
    bn_zero(bn->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_one(wrapper_bn_t* bn) {
  RLC_TRY {
    bn_set_dig(bn->value, 1);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_get_order(wrapper_bn_t* bn) {
  RLC_TRY {
    bn_null(bn->value);
    bn_new(bn->value);
    ep_curve_get_ord(bn->value);
  }
  RLC_CATCH_ANY {
    bn_free(bn->value);
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_add_assign(wrapper_bn_t* dst, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_add(dst->value, dst->value, rhs->value);
    bn_mod(dst->value, dst->value, order);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_add(wrapper_bn_t* dst, const wrapper_bn_t* lhs, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_add(dst->value, lhs->value, rhs->value);
    bn_mod(dst->value, dst->value, order);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_double(wrapper_bn_t* dst, const wrapper_bn_t* src) {
  RLC_TRY {
    bn_dbl(dst->value, src->value);
    bn_mod(dst->value, dst->value, order);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_neg(wrapper_bn_t* bn) {
  RLC_TRY {
    bn_sub(bn->value, order, bn->value);
    bn_mod(bn->value, bn->value, order);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_sub_assign(wrapper_bn_t* dst, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_sub(dst->value, dst->value, rhs->value);
    bn_mod(dst->value, dst->value, order);
    if (bn_sign(dst->value) == RLC_NEG) {
      bn_add(dst->value, dst->value, order);
    }
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_sub(wrapper_bn_t* dst, const wrapper_bn_t* lhs, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_sub(dst->value, lhs->value, rhs->value);
    bn_mod(dst->value, dst->value, order);
    if (bn_sign(dst->value) == RLC_NEG) {
      bn_add(dst->value, dst->value, order);
    }
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_mul_assign(wrapper_bn_t* dst, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_mul(dst->value, dst->value, rhs->value);
    if (bn_sign(dst->value) == RLC_NEG) {
      bn_add(dst->value, dst->value, order);
    } else {
      bn_mod(dst->value, dst->value, order);
    }
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_mul(wrapper_bn_t* dst, const wrapper_bn_t* lhs, const wrapper_bn_t* rhs) {
  RLC_TRY {
    bn_mul(dst->value, lhs->value, rhs->value);
    if (bn_sign(dst->value) == RLC_NEG) {
      bn_add(dst->value, dst->value, order);
    } else {
      bn_mod(dst->value, dst->value, order);
    }
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_inv(wrapper_bn_t* val) {
  if (bn_is_zero(val->value)) {
    return RLC_ERR;
  }
  RLC_TRY {
    bn_mod_inv(val->value, val->value, order);
    if (bn_sign(val->value) == RLC_NEG) {
      bn_add(val->value, val->value, order);
    }
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_size_bin(size_t* size, const wrapper_bn_t* bn) {
  RLC_TRY {
    *size = bn_size_bin(bn->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_write_bin(uint8_t* dst, size_t len, const wrapper_bn_t* src) {
  RLC_TRY {
    bn_write_bin(dst, len, src->value);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_read_bin(wrapper_bn_t* dst, const uint8_t* src, size_t len) {
  RLC_TRY {
    bn_read_bin(dst->value, src, len);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

int wrapper_bn_rand(wrapper_bn_t* dst, const uint8_t* src, size_t len) {
  RLC_TRY {
    bn_read_bin(dst->value, src, len);
    bn_mod(dst->value, dst->value, order);
  }
  RLC_CATCH_ANY {
    return RLC_ERR;
  }

  return RLC_OK;
}

bool wrapper_bn_is_zero(const wrapper_bn_t* value) {
  return bn_is_zero(value->value) == 1;
}

bool wrapper_bn_is_odd(const wrapper_bn_t* value) {
  return bn_is_even(value->value) == 0;
}