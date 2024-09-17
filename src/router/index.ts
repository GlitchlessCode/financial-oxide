import { createWebHistory, createRouter } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      redirect: {
        name: "sheet",
        params: {
          id: "0",
        },
      },
    },
    {
      path: "/load",
      name: "load",
      component: () => import("../views/LoadingView.vue"),
      meta: {
        transition: "slide-down",
      },
    },
    {
      path: "/edit",
      name: "edit",
      component: () => import("../views/EditingView.vue"),
      children: [
        {
          path: ":id",
          name: "sheet",
          component: () => import("../views/editing/SheetView.vue"),
          props: true,
        },
      ],
      meta: {
        transition: "slide-up",
      },
    },
  ],
});

export default router;
