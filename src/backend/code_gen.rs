use inkwell::context::Context;
use inkwell::values::{BasicValueEnum};
use inkwell::types::BasicTypeEnum;
use inkwell::builder::Builder;
use inkwell::module::Module;
use crate::*;

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        Self {
            context,
            builder,
            module,
        }
    }

    pub fn gen_expr(&self, expr: &Expr) -> BasicValueEnum<'ctx> {
        match expr {
            Expr::Int(i) => {
                let int_type = self.context.i64_type();
                int_type.const_int(*i as u64, true).into()
            }
            Expr::Binary { left, op, right } => {
                let left_val = self.gen_expr(left).into_int_value();
                let right_val = self.gen_expr(right).into_int_value();
                let res = match op.as_str() {
                    "+" => self.builder.build_int_add(left_val, right_val, "addtmp"),
                    "-" => self.builder.build_int_sub(left_val, right_val, "subtmp"),
                    "*" => self.builder.build_int_mul(left_val, right_val, "multmp"),
                    "/" => self.builder.build_int_signed_div(left_val, right_val, "divtmp"),
                    _ => panic!("unsupported binary operator"),
                };
                res.into()
            }
        }
    }

    pub fn gen_stmt(&self, stmt: &Stmt) -> Option<BasicValueEnum<'ctx>> {
        match stmt {
            Stmt::Expr(expr) => Some(self.gen_expr(expr)),
            Stmt::Var { name: _, value } => Some(self.gen_expr(value)),
            Stmt::Const { name: _, value } => Some(self.gen_expr(value)),
            Stmt::Package(_) => None,
            Stmt::Import(_) => None,
            Stmt::Return(expr) => Some(self.gen_expr(expr)),
        }
    }

    pub fn ptr_type(&self, ty: BasicTypeEnum<'ctx>) -> inkwell::types::PointerType<'ctx> {
        ty.ptr_type(AddressSpace::Generic)
    }

    pub fn array_type(&self, ty: BasicTypeEnum<'ctx>, size: u32) -> inkwell::types::ArrayType<'ctx> {
        ty.array_type(size)
    }

    pub fn void_type(&self) -> VoidType<'ctx> {
        self.context.void_type()
    }
}